use near_sdk::{borsh::BorshSerialize, AccountId, FunctionError, NearToken};
use thiserror::Error;

#[derive(BorshSerialize, Debug, Error, FunctionError)]
#[borsh(crate = "near_sdk::borsh")]
pub enum ContractError {
    #[error("Contract already initialized")]
    AlreadyInitilized,
    #[error("Account {} not registered", _0)]
    AccountNotRegistered(AccountId),
    #[error("Unable to deserialize message")]
    Deserialize,
    #[error("Received token is not SHITZU")]
    TokenNotShitzu,
    #[error("Claim fees can't run in parallel")]
    ClaimFeesLocked,
    #[error("Not enough NEAR deposit. Required: {}, actual: {}", _0, _1)]
    NotEnoughDeposit(NearToken, NearToken),
    #[error("Operation not supported")]
    OperationNotSupported,
}
