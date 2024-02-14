pub mod call;
pub mod event;
pub mod view;

use near_sdk::{json_types::U128, serde_json};
use near_workspaces::{
    network::Sandbox,
    result::{ExecutionFinalResult, ExecutionResult, Value, ViewResultDetails},
    types::{KeyType, SecretKey},
    Account, Contract, Worker,
};
use owo_colors::OwoColorize;

const WASM_BINARY: &[u8] = include_bytes!("../../../../res/contract.wasm");
const TEST_TOKEN_BINARY: &[u8] = include_bytes!("../../../../res/test_token.wasm");

#[macro_export]
macro_rules! print_log {
    ( $x:expr, $($y:expr),+ ) => {
        let thread_name = std::thread::current().name().unwrap().to_string();
        if thread_name == "main" {
            println!($x, $($y),+);
        } else {
            let mut s = format!($x, $($y),+);
            s = s.split('\n').map(|s| {
                let mut pre = "    ".to_string();
                pre.push_str(s);
                pre.push('\n');
                pre
            }).collect::<String>();
            println!(
                "{}\n{}",
                thread_name.bold(),
                &s[..s.len() - 1],
            );
        }
    };
}

pub async fn initialize_contracts(
    fee: u128,
) -> anyhow::Result<(Worker<Sandbox>, Account, Contract, Contract)> {
    let worker = near_workspaces::sandbox().await?;

    let owner = worker.dev_create_account().await?;

    let token_contract = worker.dev_deploy(TEST_TOKEN_BINARY).await?;
    token_contract
        .call("new")
        .args_json(("Shitzu", "SHITZU", 18))
        .transact()
        .await?
        .into_result()?;

    let key = SecretKey::from_random(KeyType::ED25519);
    let contract = worker
        .create_tla_and_deploy("contract.test.near".parse()?, key, WASM_BINARY)
        .await?
        .into_result()?;

    contract
        .call("new")
        .args_json((owner.id(), token_contract.id(), U128(fee)))
        .max_gas()
        .transact()
        .await?
        .into_result()?;

    Ok((worker, owner, contract, token_contract))
}

pub fn log_tx_result(
    ident: &str,
    res: ExecutionFinalResult,
) -> anyhow::Result<(ExecutionResult<Value>, Vec<event::ContractEvent>)> {
    for failure in res.receipt_failures() {
        print_log!("{:#?}", failure.bright_red());
    }
    let mut events = vec![];
    for outcome in res.receipt_outcomes() {
        if !outcome.logs.is_empty() {
            for log in outcome.logs.iter() {
                if log.starts_with("EVENT_JSON:") {
                    let event: event::ContractEvent =
                        serde_json::from_str(&log.replace("EVENT_JSON:", ""))?;
                    events.push(event.clone());
                    print_log!(
                        "{}: {}\n{}",
                        "account".bright_cyan(),
                        outcome.executor_id,
                        event
                    );
                } else {
                    print_log!("{}", log.bright_yellow());
                }
            }
        }
    }
    print_log!(
        "{} gas burnt: {:.3} {}",
        ident.italic(),
        res.total_gas_burnt.as_tgas().bright_magenta().bold(),
        "TGas".bright_magenta().bold()
    );
    Ok((res.into_result()?, events))
}

pub fn log_view_result(res: ViewResultDetails) -> anyhow::Result<ViewResultDetails> {
    if !res.logs.is_empty() {
        for log in res.logs.iter() {
            print_log!("{}", log.bright_yellow());
        }
    }
    Ok(res)
}
