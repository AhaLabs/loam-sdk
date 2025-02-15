use loam_sdk::{
    loamstorage, soroban_sdk::{self, contracttype, env, Address, Lazy, PersistentItem}, subcontract
};

use crate::error::Error;

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
#[derive(Clone)]
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
            seller: env().current_contract_address(),
            sell_token: env().current_contract_address(),
            buy_token: env().current_contract_address(),
            sell_price: 0,
            buy_price: 0,
        }
    }
}

#[loamstorage]
pub struct Storage {
    offer: PersistentItem<SingleOffer>,
}

#[subcontract]
pub trait IsSingleOfferTrait {
    fn create(
        &mut self,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    ) -> Result<(), Error>;
    fn trade(
        &self,
        buyer: Address,
        buy_token_amount: i128,
        min_sell_token_amount: i128,
    ) -> Result<(), Error>;
    fn withdraw(&self, token: Address, amount: i128) -> Result<(), Error>;
    fn update_price(&mut self, sell_price: u32, buy_price: u32) -> Result<(), Error>;
    fn get_offer(&self) -> SingleOffer;
}

impl IsSingleOfferTrait for Storage {
    fn create(
        &mut self,
        seller: Address,
        sell_token: Address,
        buy_token: Address,
        sell_price: u32,
        buy_price: u32,
    ) -> Result<(), Error> {
        if self.offer.get().is_some() {
            return Err(Error::OfferAlreadyCreated);
        }
        if buy_price == 0 || sell_price == 0 {
            return Err(Error::ZeroPriceNotAllowed);
        }
        seller.require_auth();
        let offer = SingleOffer {
            seller,
            sell_token,
            buy_token,
            sell_price,
            buy_price,
        };
        self.offer.set(&offer);
        Ok(())
    }

    // Trades `buy_token_amount` of buy_token from buyer for `sell_token` amount
    // defined by the price.
    // `min_sell_amount` defines a lower bound on the price that the buyer would
    // accept.
    // Buyer needs to authorize the `trade` call and internal `transfer` call to
    // the contract address.
    fn trade(
        &self,
        buyer: Address,
        buy_token_amount: i128,
        min_sell_token_amount: i128,
    ) -> Result<(), Error> {
        // Buyer needs to authorize the trade.
        buyer.require_auth();
        let offer = self.offer.get().unwrap();

        // prepare the token clients to do the trade.
        let sell_token_client = soroban_sdk::token::Client::new(env(), &offer.sell_token.clone());
        let buy_token_client = soroban_sdk::token::Client::new(env(), &offer.buy_token.clone());

        let sell_token_amount = buy_token_amount
            .checked_mul(i128::from(offer.sell_price))
            .unwrap()
            / i128::from(offer.buy_price);

        if sell_token_amount < min_sell_token_amount {
            return Err(Error::PriceTooLow);
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
        buy_token_client.transfer(&contract, &offer.seller, &buy_token_amount);

        Ok(())
    }

    fn withdraw(&self, token: Address, amount: i128) -> Result<(), Error> {
        let SingleOffer { seller, ..} = self.offer.get().unwrap();
        seller.require_auth();
        soroban_sdk::token::Client::new(env(), &token).transfer(
            &env().current_contract_address(),
            &seller,
            &amount,
        );
        Ok(())
    }

    fn update_price(&mut self, sell_price: u32, buy_price: u32) -> Result<(), Error> {
        let mut offer = self.offer.get().unwrap();
        if buy_price == 0 || sell_price == 0 {
            return Err(Error::ZeroPriceNotAllowed);
        }
        offer.seller.require_auth();
        offer.sell_price = sell_price;
        offer.buy_price = buy_price;
        self.offer.set(&offer);
        Ok(())
    }

    fn get_offer(&self) -> SingleOffer {
        self.offer.get().unwrap()
    }
}
