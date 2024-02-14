use crate::log_view_result;
use near_sdk::{json_types::U128, AccountId};
use near_workspaces::Contract;

pub async fn ft_balance_of(contract: &Contract, account_id: &AccountId) -> anyhow::Result<U128> {
    let res = log_view_result(
        contract
            .call("ft_balance_of")
            .args_json((account_id,))
            .max_gas()
            .view()
            .await?,
    )?;
    Ok(res.json()?)
}

pub async fn get_collected_fees(contract: &Contract) -> anyhow::Result<U128> {
    let res = log_view_result(contract.call("get_collected_fees").max_gas().view().await?)?;
    Ok(res.json()?)
}

pub async fn get_balance(contract: &Contract, account_id: &AccountId) -> anyhow::Result<U128> {
    let res = log_view_result(
        contract
            .call("get_balance")
            .args_json((account_id,))
            .max_gas()
            .view()
            .await?,
    )?;
    Ok(res.json()?)
}
