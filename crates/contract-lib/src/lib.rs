mod error;
mod ft_receiver;
mod storage;

pub use error::*;
pub use ft_receiver::*;
pub use storage::*;

use near_contract_standards::fungible_token::core::ext_ft_core;
use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    near_bindgen,
    store::UnorderedMap,
    AccountId, BorshStorageKey, Gas, NearToken, PanicOnDefault, Promise, PromiseResult,
};

#[derive(BorshStorageKey, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub enum StorageKey {
    Accounts,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    pub owner: AccountId,
    pub shitzu_token_id: AccountId,
    pub fee: u128,
    pub collected_fees: u128,
    pub lock_fee_sending: bool,
    pub accounts: UnorderedMap<AccountId, u128>,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[handle_result]
    pub fn new(
        owner: AccountId,
        shitzu_token_id: AccountId,
        fee: U128,
    ) -> Result<Self, ContractError> {
        if env::state_exists() {
            return Err(ContractError::AlreadyInitilized);
        }
        Ok(Self {
            owner,
            shitzu_token_id,
            fee: fee.0,
            collected_fees: 0,
            lock_fee_sending: false,
            accounts: UnorderedMap::new(StorageKey::Accounts),
        })
    }

    #[private]
    pub fn deduct_fee(&mut self, account_id: AccountId) {
        let balance = self.accounts.get_mut(&account_id).unwrap();
        *balance = balance.checked_sub(self.fee).unwrap();
        self.collected_fees += self.fee;
    }

    #[private]
    pub fn refund_fee(&mut self, account_id: AccountId) {
        let balance = self.accounts.get_mut(&account_id).unwrap();
        *balance += self.fee;
        self.collected_fees -= self.fee;
    }

    #[handle_result]
    pub fn withdraw(&mut self, amount: U128) -> Result<Promise, ContractError> {
        let sender_id = env::predecessor_account_id();
        if !self.accounts.contains_key(&sender_id) {
            return Err(ContractError::AccountNotRegistered(sender_id));
        }
        let balance = self.accounts.get_mut(&sender_id).unwrap();
        *balance -= amount.0;

        Ok(ext_ft_core::ext(self.shitzu_token_id.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(15))
            .ft_transfer(sender_id.clone(), amount, Some("withdraw".to_string()))
            .then(
                Self::ext(env::current_account_id())
                    .with_unused_gas_weight(1)
                    .after_withdraw(sender_id, amount),
            ))
    }

    #[private]
    #[handle_result]
    pub fn after_withdraw(
        &mut self,
        sender_id: AccountId,
        amount: U128,
    ) -> Result<(), ContractError> {
        if let PromiseResult::Failed = env::promise_result(0) {
            let balance = self.accounts.get_mut(&sender_id).unwrap();
            *balance += amount.0;
        }

        Ok(())
    }

    #[handle_result]
    pub fn claim_fees(&mut self) -> Result<Promise, ContractError> {
        if self.lock_fee_sending {
            return Err(ContractError::ClaimFeesLocked);
        }
        let min_collected_fees = 5 * self.fee;
        let collected_fees_refund = self.collected_fees;
        let collected_fees = self.collected_fees.checked_sub(min_collected_fees).unwrap();
        self.collected_fees = min_collected_fees;
        self.lock_fee_sending = true;
        Ok(ext_ft_core::ext(self.shitzu_token_id.clone())
            .with_static_gas(Gas::from_tgas(15))
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .ft_transfer(self.owner.clone(), collected_fees.into(), None)
            .then(
                Self::ext(env::current_account_id())
                    .with_unused_gas_weight(1)
                    .after_claim_fees(collected_fees_refund.into()),
            ))
    }

    #[private]
    #[handle_result]
    pub fn after_claim_fees(&mut self, collected_fees: U128) -> Result<(), ContractError> {
        self.lock_fee_sending = false;
        if let PromiseResult::Failed = env::promise_result(0) {
            self.collected_fees = collected_fees.0;
        }

        Ok(())
    }

    pub fn get_collected_fees(&self) -> U128 {
        self.collected_fees.into()
    }

    pub fn get_balance(&self, account_id: AccountId) -> U128 {
        (*self.accounts.get(&account_id).unwrap()).into()
    }
}
