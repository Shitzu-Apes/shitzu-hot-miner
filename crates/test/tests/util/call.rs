use contract_lib::FtReceiverMsg;
use near_sdk::{json_types::U128, serde::Serialize, serde_json::json, AccountId, Gas, NearToken};
use near_workspaces::{
    result::{ExecutionFinalResult, ExecutionResult, Value},
    Account, Contract,
};

use crate::{event, log_tx_result};

pub async fn storage_deposit(
    contract: &Contract,
    sender: &Account,
    account_id: Option<&AccountId>,
    deposit: Option<NearToken>,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        "storage_deposit",
        sender
            .call(contract.id(), "storage_deposit")
            .args_json((account_id, None::<bool>))
            .deposit(deposit.unwrap_or(NearToken::from_millinear(10)))
            .max_gas()
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn mint_tokens(
    token: &Contract,
    receiver: &AccountId,
    amount: u128,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        "mint",
        token
            .call("mint")
            .args_json((receiver, U128::from(amount)))
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn withdraw(
    contract: &Contract,
    sender: &Account,
    amount: u128,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        "withdraw",
        sender
            .call(contract.id(), "withdraw")
            .args_json((U128::from(amount),))
            .gas(Gas::from_tgas(50))
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn deduct_fee(
    contract: &Contract,
    account_id: &AccountId,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        "deduct_fee",
        contract
            .call("deduct_fee")
            .args_json((account_id,))
            .gas(Gas::from_tgas(50))
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn refund_fee(
    contract: &Contract,
    account_id: &AccountId,
) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        "refund_fee",
        contract
            .call("refund_fee")
            .args_json((account_id,))
            .gas(Gas::from_tgas(50))
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn claim_fees(contract: &Contract) -> anyhow::Result<ExecutionResult<Value>> {
    let (res, _) = log_tx_result(
        "claim_fees",
        contract
            .call("claim_fees")
            .gas(Gas::from_tgas(50))
            .transact()
            .await?,
    )?;
    Ok(res)
}

pub async fn deposit(
    sender: &Account,
    token_id: &AccountId,
    receiver_id: &AccountId,
    amount: u128,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    let (res, events): (ExecutionResult<Value>, Vec<event::ContractEvent>) = log_tx_result(
        "deposit",
        ft_transfer_call(
            sender,
            token_id,
            receiver_id,
            U128(amount),
            FtReceiverMsg::Deposit,
        )
        .await?,
    )?;
    Ok((res, events))
}

async fn ft_transfer_call<T: Serialize>(
    sender: &Account,
    token_id: &AccountId,
    receiver_id: &AccountId,
    amount: U128,
    msg: T,
) -> anyhow::Result<ExecutionFinalResult> {
    Ok(sender
        .call(token_id, "ft_transfer_call")
        .args_json((
            receiver_id,
            amount,
            Option::<String>::None,
            json!(msg).to_string(),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?)
}

// TODO send to DAO
