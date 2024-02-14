use crate::{Contract, ContractError, ContractExt};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    env,
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PromiseOrValue,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum FtReceiverMsg {
    Deposit,
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        match self.internal_ft_on_transfer(sender_id, amount, msg) {
            Ok(res) => res,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

impl Contract {
    fn internal_ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> Result<PromiseOrValue<U128>, ContractError> {
        let msg: FtReceiverMsg =
            near_sdk::serde_json::from_str(&msg).map_err(|_| ContractError::Deserialize)?;
        let token_id = env::predecessor_account_id();
        if token_id != self.shitzu_token_id {
            return Err(ContractError::TokenNotShitzu);
        }

        match msg {
            FtReceiverMsg::Deposit => {
                let balance = self
                    .accounts
                    .get_mut(&sender_id)
                    .ok_or_else(|| ContractError::AccountNotRegistered(sender_id))?;
                *balance += amount.0;
            }
        }

        Ok(PromiseOrValue::Value(0.into()))
    }
}
