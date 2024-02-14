use crate::{Contract, ContractError, ContractExt};
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};
use near_sdk::{assert_one_yocto, env, near_bindgen, AccountId, NearToken, Promise};

pub const STORAGE_ACCOUNT_COST: NearToken = NearToken::from_millinear(10);

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        match self.internal_storage_deposit(account_id, registration_only) {
            Ok(res) => res,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<NearToken>) -> StorageBalance {
        assert_one_yocto();
        match self.internal_storage_withdraw(amount) {
            Ok(res) => res,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        assert_one_yocto();
        match self.internal_storage_unregister(force) {
            Ok(res) => res,
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        StorageBalanceBounds {
            min: STORAGE_ACCOUNT_COST,
            max: None,
        }
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.accounts.get(&account_id).map(|_| StorageBalance {
            total: STORAGE_ACCOUNT_COST,
            available: NearToken::from_yoctonear(0),
        })
    }
}

impl Contract {
    fn internal_storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        _registration_only: Option<bool>,
    ) -> Result<StorageBalance, ContractError> {
        let amount = env::attached_deposit();
        let account_id = account_id.unwrap_or_else(env::predecessor_account_id);
        let min_balance = self.storage_balance_bounds().min;
        let already_registered = self.accounts.contains_key(&account_id);
        if amount < min_balance && !already_registered {
            return Err(ContractError::NotEnoughDeposit(min_balance, amount));
        }
        if already_registered {
            if amount.as_yoctonear() > 0 {
                Promise::new(env::predecessor_account_id()).transfer(amount);
            }
            Ok(self.storage_balance_of(account_id).unwrap())
        } else {
            let refund = amount.checked_sub(min_balance).unwrap();
            if refund.as_yoctonear() > 0 {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
            self.accounts.insert(account_id, 0);
            Ok(StorageBalance {
                total: STORAGE_ACCOUNT_COST,
                available: NearToken::from_yoctonear(0),
            })
        }
    }

    fn internal_storage_withdraw(
        &mut self,
        _amount: Option<NearToken>,
    ) -> Result<StorageBalance, ContractError> {
        Err(ContractError::OperationNotSupported)
    }

    fn internal_storage_unregister(&mut self, force: Option<bool>) -> Result<bool, ContractError> {
        if force.is_some() {
            return Err(ContractError::OperationNotSupported);
        }
        let account_id = env::predecessor_account_id();
        if self.accounts.contains_key(&account_id) {
            Promise::new(account_id.clone()).transfer(STORAGE_ACCOUNT_COST);
            self.accounts.remove(&account_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
