use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, Bytes, Env, IntoKey, Lazy, Map},
    subcontract,
};

use crate::error::Error;

fn check_nonnegative_amount(amount: i128) -> Result<(), Error> {
    if amount < 0 {
        Err(Error::InsufficientBalance)
    } else {
        Ok(())
    }
}

fn read_allowance(e: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = e.storage().temporary().get::<_, AllowanceValue>(&key) {
        if allowance.expiration_ledger < e.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}


pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    State(Address),
    Admin,
}

#[contracttype]
#[derive(IntoKey, Clone)]
pub struct Token {
    admin: Address,
    decimal: u32,
    name: Bytes,
    symbol: Bytes,
    balances: Map<Address, i128>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            admin: env().current_contract_address(),
            decimal: 0,
            name: Bytes::from_array(env(), &[]),
            symbol: Bytes::from_array(env(), &[]),
            balances: Map::new(env()),
        }
    }
}

#[subcontract]
pub trait IsTokenTrait {
    fn initialize(&mut self, admin: Address, decimal: u32, name: Bytes, symbol: Bytes) -> Result<(), Error>;
    fn allowance(&self, from: Address, spender: Address) -> i128;
    fn balance(&self, id: Address) -> i128;
    fn transfer(&mut self, from: Address, to: Address, amount: i128) -> Result<(), Error>;
    fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: i128) -> Result<(), Error>;
    fn burn(&mut self, from: Address, amount: i128) -> Result<(), Error>;
    fn burn_from(&mut self, spender: Address, from: Address, amount: i128) -> Result<(), Error>;
    fn mint(&mut self, to: Address, amount: i128);
    fn set_admin(&mut self, new_admin: Address);
    fn decimals(&self) -> u32;
    fn name(&self) -> Bytes;
    fn symbol(&self) -> Bytes;
    fn approve(&self, from: Address, spender: Address, amount: i128, expiration_ledger: u32) -> Result<(), Error>;
}

impl IsTokenTrait for Token {
    fn initialize(&mut self, admin: Address, decimal: u32, name: Bytes, symbol: Bytes) -> Result<(), Error> {
        if self.admin != env().current_contract_address() {
            return Err(Error::AlreadyInitialized);
        }
        self.admin = admin;
        self.decimal = decimal;
        self.name = name;
        self.symbol = symbol;
        self.balances = Map::new(env());
        Ok(())
    }

    fn allowance(&self, from: Address, spender: Address) -> i128 {
        env().storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(env(), from, spender).amount
    }

    fn balance(&self, id: Address) -> i128 {
        self.balances.get(id).unwrap_or(0)
    }

    fn transfer(&mut self, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        let from_balance = self.balance(from.clone());
        let to_balance = self.balance(to.clone());
        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }
        self.balances.set(from, from_balance - amount);
        self.balances.set(to, to_balance + amount);
        Ok(())
    }

    fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        spender.require_auth();

        check_nonnegative_amount(amount)?;

        env().storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(env(), from.clone(), spender, amount)?;
        let from_balance = self.balance(from.clone());
        let to_balance = self.balance(to.clone());
        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }
        self.balances.set(from, from_balance - amount);
        self.balances.set(to, to_balance + amount);
        Ok(())
    }

    fn burn(&mut self, from: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        let balance = self.balance(from.clone());
        if balance < amount {
            return Err(Error::InsufficientBalance);
        }
        self.balances.set(from, balance - amount);
        Ok(())
    }

    fn burn_from(&mut self, spender: Address, from: Address, amount: i128) -> Result<(), Error> {
        spender.require_auth();

        check_nonnegative_amount(amount)?;

        env().storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(env(), from.clone(), spender, amount)?;
        let balance = self.balance(from.clone());
        if balance < amount {
            return Err(Error::InsufficientBalance);
        }
        self.balances.set(from, balance - amount);
        Ok(())
    }

    fn mint(&mut self, to: Address, amount: i128) {
        self.admin.require_auth();
        let balance = self.balance(to.clone());
        self.balances.set(to, balance + amount);
    }

    fn set_admin(&mut self, new_admin: Address) {
        self.admin.require_auth();
        self.admin = new_admin;
    }

    fn decimals(&self) -> u32 {
        self.decimal
    }

    fn name(&self) -> Bytes {
        self.name.clone()
    }

    fn symbol(&self) -> Bytes {
        self.symbol.clone()
    }

    fn approve(&self, from: Address, spender: Address, amount: i128, expiration_ledger: u32) -> Result<(), Error> {
        from.require_auth();

        check_nonnegative_amount(amount)?;

        env().storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        write_allowance(env(), from, spender, amount, expiration_ledger)?;
        Ok(())
    }
}


pub fn write_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) -> Result<(), Error> {
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    if amount > 0 && expiration_ledger < e.ledger().sequence() {
        return Err(Error::ExpirationInPast);
    }

    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.storage().temporary().set(&key.clone(), &allowance);

    if amount > 0 {
        let live_for = expiration_ledger
            .checked_sub(e.ledger().sequence())
            .ok_or(Error::ExpirationOverflow)?;

        e.storage().temporary().extend_ttl(&key, live_for, live_for)
    }
    Ok(())
}

pub fn spend_allowance(e: &Env, from: Address, spender: Address, amount: i128) -> Result<(), Error> {
    let allowance = read_allowance(e, from.clone(), spender.clone());
    if allowance.amount < amount {
        return Err(Error::InsufficientAllowance);
    }
    if amount > 0 {
        write_allowance(
            e,
            from,
            spender,
            allowance.amount - amount,
            allowance.expiration_ledger,
        )?;
    }
    Ok(())
}