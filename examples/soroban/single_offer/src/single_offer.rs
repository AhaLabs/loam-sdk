use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, Bytes, IntoKey, Lazy},
    subcontract,
};

/*
How this contract should be used:

1. Call `create` once to create the offer and register its seller.
2. Seller may transfer arbitrary amounts of the `sell_token` for sale to the
   contract address for trading. They may also update the offer price.
3. Buyers may call `trade` to trade with the offer. The contract will
   immediately perform the trade and send the respective amounts of `buy_token`
   and `sell_token` to the seller and buyer respectively.
4. Seller may call `withdraw` to claim any remaining `sell_token` balance.
*/
#[contracttype]
#[derive(IntoKey, Clone)]
pub struct SingleOffer {
    seller: Address,
    sell_token: Address,
    buy_token: Address,
    sell_price: u32,
    buy_price: u32,
}

impl Default for SingleOffer {
    fn default() -> Self {
        Self {
            seller: Address::from_string_bytes(&Bytes::from_array(env(), &[0; 32])),
            sell_token: Address::from_string_bytes(&Bytes::from_array(env(), &[0; 32])),
            buy_token: Address::from_string_bytes(&Bytes::from_array(env(), &[0; 32])),
            sell_price: 0,
            buy_price: 0,
        }
    }
}

#[subcontract]
pub trait IsSingleOfferTrait {
    // Creates the offer for seller for the given token pair and initial price.
    // See comment above the `Offer` struct for information on pricing.
    fn create(
        &mut self,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    );
    fn trade(&self, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128);
    fn withdraw(&self, token: Address, amount: i128);
    fn updt_price(&mut self, sell_price: u32, buy_price: u32);
    fn get_offer(&self) -> SingleOffer;
}

impl IsSingleOfferTrait for SingleOffer {
    fn create(
        &mut self,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    ) {
        if self.sell_price != 0 || self.buy_price != 0 {
            panic!("offer is already created");
        }
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        seller.require_auth();
        
        self.seller = seller;
        self.sell_token = sell_token;
        self.buy_token = buy_token;
        self.sell_price = sell_price;
        self.buy_price = buy_price;
    }

    // Trades `buy_token_amount` of buy_token from buyer for `sell_token` amount
    // defined by the price.
    // `min_sell_amount` defines a lower bound on the price that the buyer would
    // accept.
    // Buyer needs to authorize the `trade` call and internal `transfer` call to
    // the contract address.
    fn trade(&self, buyer: Address, buy_token_amount: i128, min_sell_token_amount: i128) {
        buyer.require_auth();

        let sell_token_client = soroban_sdk::token::Client::new(&env(), &self.sell_token.clone());
        let buy_token_client = soroban_sdk::token::Client::new(&env(), &self.buy_token.clone());

        let sell_token_amount = buy_token_amount
            .checked_mul(self.sell_price as i128)
            .unwrap()
            / self.buy_price as i128;

        if sell_token_amount < min_sell_token_amount {
            panic!("price is too low");
        }

        let contract = env().current_contract_address();

        // Perform the trade in 3 `transfer` steps.
        // Note, that we don't need to verify any balances - the contract would
        // just trap and roll back in case if any of the transfers fails for
        // any reason, including insufficient balance.

        // Transfer the `buy_token` from buyer to this contract.
        // This `transfer` call should be authorized by buyer.
        // This could as well be a direct transfer to the seller, but sending to
        // the contract address allows building more transparent signature
        // payload where the buyer doesn't need to worry about sending token to
        // some 'unknown' third party.
        buy_token_client.transfer(&buyer, &contract, &buy_token_amount);
        // Transfer the `sell_token` from contract to buyer.
        sell_token_client.transfer(&contract, &buyer, &sell_token_amount);
        // Transfer the `buy_token` to the seller immediately.
        buy_token_client.transfer(&contract, &self.seller, &buy_token_amount);
    }

    // Sends amount of token from this contract to the seller.
    // This is intentionally flexible so that the seller can withdraw any
    // outstanding balance of the contract (in case if they mistakenly
    // transferred wrong token to it).
    // Must be authorized by seller.
    fn withdraw(&self, token: Address, amount: i128) {
        self.seller.require_auth();
        soroban_sdk::token::Client::new(&env(), &token).transfer(
            &env().current_contract_address(),
            &self.seller,
            &amount,
        );
    }

    fn updt_price(&mut self, sell_price: u32, buy_price: u32) {
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }
        self.seller.require_auth();
        self.sell_price = sell_price;
        self.buy_price = buy_price;
    }

    // Returns the current state of the offer.
    fn get_offer(&self) -> SingleOffer {
        self.clone()
    }
}
