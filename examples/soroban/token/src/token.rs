use loam_sdk::{
    soroban_sdk::{self, contracttype, env, panic_with_error, Address, Bytes, Env, IntoKey, Lazy, Map},
    subcontract,
};

use crate::error::Error;

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
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
    fn initialize(&mut self, admin: Address, decimal: u32, name: Bytes, symbol: Bytes);
    fn allowance(&self, from: Address, spender: Address) -> i128;
    fn balance(&self, id: Address) -> i128;
    fn transfer(&mut self, from: Address, to: Address, amount: i128);
    fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: i128);
    fn burn(&mut self, from: Address, amount: i128);
    fn burn_from(&mut self, spender: Address, from: Address, amount: i128);
    fn mint(&mut self, to: Address, amount: i128);
    fn set_admin(&mut self, new_admin: Address);
    fn decimals(&self) -> u32;
    fn name(&self) -> Bytes;
    fn symbol(&self) -> Bytes;
    fn approve(&self, from: Address, spender: Address, amount: i128, expiration_ledger: u32);
}

impl IsTokenTrait for Token {
    fn initialize(&mut self, admin: Address, decimal: u32, name: Bytes, symbol: Bytes) {
        if self.admin != env().current_contract_address() {
            panic!("already initialized");
        }
        self.admin = admin;
        self.decimal = decimal;
        self.name = name;
        self.symbol = symbol;
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

    fn transfer(&mut self, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let from_balance = self.balance(from.clone());
        let to_balance = self.balance(to.clone());
        if from_balance < amount {
            panic_with_error!(env(), Error::InsufficientBalance);
        }
        self.balances.set(from, from_balance - amount);
        self.balances.set(to, to_balance + amount);
    }

    fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        env().storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(env(), from.clone(), spender, amount);
        let from_balance = self.balance(from.clone());
        let to_balance = self.balance(to.clone());
        if from_balance < amount {
            panic!("insufficient balance");
        }
        self.balances.set(from, from_balance - amount);
        self.balances.set(to, to_balance + amount);
    }

    fn burn(&mut self, from: Address, amount: i128) {
        from.require_auth();
        let balance = self.balance(from.clone());
        if balance < amount {
            panic!("insufficient balance");
        }
        self.balances.set(from, balance - amount);
    }

    fn burn_from(&mut self, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        env().storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(env(), from.clone(), spender, amount);
        let balance = self.balance(from.clone());
        if balance < amount {
            panic!("insufficient balance");
        }
        self.balances.set(from, balance - amount);
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

    fn approve(&self, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        env().storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        write_allowance(env(), from, spender, amount, expiration_ledger);
    }
}


pub fn write_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    if amount > 0 && expiration_ledger < e.ledger().sequence() {
        panic!("expiration_ledger is less than ledger seq when amount > 0")
    }

    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    e.storage().temporary().set(&key.clone(), &allowance);

    if amount > 0 {
        let live_for = expiration_ledger
            .checked_sub(e.ledger().sequence())
            .unwrap();

        e.storage().temporary().extend_ttl(&key, live_for, live_for)
    }
}

pub fn spend_allowance(e: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read_allowance(e, from.clone(), spender.clone());
    if allowance.amount < amount {
        panic!("insufficient allowance");
    }
    if amount > 0 {
        write_allowance(
            e,
            from,
            spender,
            allowance.amount - amount,
            allowance.expiration_ledger,
        );
    }
}

