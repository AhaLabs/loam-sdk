use loam_sdk::{
    soroban_sdk::{self, contracttype, Address, env, IntoKey, Bytes, Map, Lazy},
    subcontract,
};

#[contracttype]
#[derive(IntoKey, Clone)]
pub struct Token {
    admin: Address,
    decimal: u32,
    name: Bytes,
    symbol: Bytes,
    balances: Map<Address, i128>,
    allowances: Map<(Address, Address), i128>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            admin: env().current_contract_address(),
            decimal: 0,
            name: Bytes::from_array(env(), &[]),
            symbol: Bytes::from_array(env(), &[]),
            balances: Map::new(env()),
            allowances: Map::new(env()),
        }
    }
}

#[subcontract]
pub trait IsTokenTrait {
    fn initialize(&mut self, admin: Address, decimal: u32, name: Bytes, symbol: Bytes);
    fn allowance(&self, from: Address, spender: Address) -> i128;
    fn increase_allowance(&mut self, from: Address, spender: Address, amount: i128);
    fn decrease_allowance(&mut self, from: Address, spender: Address, amount: i128);
    fn balance(&self, id: Address) -> i128;
    fn transfer(&mut self, from: Address, to: Address, amount: i128);
    fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: i128);
    fn burn(&mut self, from: Address, amount: i128);
    fn mint(&mut self, to: Address, amount: i128);
    fn set_admin(&mut self, new_admin: Address);
    fn decimals(&self) -> u32;
    fn name(&self) -> Bytes;
    fn symbol(&self) -> Bytes;
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
        self.allowances.get((from, spender)).unwrap_or(0)
    }

    fn increase_allowance(&mut self, from: Address, spender: Address, amount: i128) {
        from.require_auth();
        let key = (from, spender);
        let current = self.allowances.get(key.clone()).unwrap_or(0);
        let new_amount = current.checked_add(amount).expect("allowance overflow");
        self.allowances.set(key, new_amount);
    }

    fn decrease_allowance(&mut self, from: Address, spender: Address, amount: i128) {
        from.require_auth();
        let key = (from, spender);
        let current = self.allowances.get(key.clone()).unwrap_or(0);
        let new_amount = current.saturating_sub(amount);
        self.allowances.set(key, new_amount);
    }

    fn balance(&self, id: Address) -> i128 {
        self.balances.get(id).unwrap_or(0)
    }

    fn transfer(&mut self, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let from_balance = self.balance(from.clone());
        let to_balance = self.balance(to.clone());
        if from_balance < amount {
            panic!("insufficient balance");
        }
        self.balances.set(from, from_balance - amount);
        self.balances.set(to, to_balance + amount);
    }

    fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        let allowance = self.allowance(from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }
        self.transfer(from.clone(), to, amount);
        self.decrease_allowance(from, spender, amount);
    }

    fn burn(&mut self, from: Address, amount: i128) {
        from.require_auth();
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
}
