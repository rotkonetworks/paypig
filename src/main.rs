use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use std::{env, fs};
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::{sr25519::Keypair, SecretUri};

#[subxt::subxt(runtime_metadata_path = "polkadot.scale")]
pub mod polkadot {}

#[derive(Parser)]
#[command(name = "paypig", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Dryrun,
    Pay,
}

struct Payable {
    index: u32,
    amount: u128,
    beneficiary: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let node = env::var("WS_NODE")?;
    let client = OnlineClient::<PolkadotConfig>::from_url(&node).await?;
    let cmd = Cli::parse().cmd;

    match cmd {
        Cmd::Dryrun => {
            let payables = find_payables(&client).await?;
            display_payables(&payables);
        }
        Cmd::Pay => {
            let payables = find_payables(&client).await?;
            display_payables(&payables);

            if payables.is_empty() {
                println!("⚠️ No payouts found.");
                return Ok(());
            }

            let key = fs::read_to_string(".keyfile")?.trim().to_string();
            let suri: SecretUri = key.parse()?;
            let keypair = Keypair::from_uri(&suri)?;

            if payables.len() == 1 {
                execute_payment(&client, &keypair, &payables[0]).await?;
            } else {
                execute_payments(&client, &keypair, &payables).await?;
            }
        }
    }

    Ok(())
}

async fn find_payables(api: &OnlineClient<PolkadotConfig>) -> Result<Vec<Payable>> {
    let block_ref = api.backend().latest_finalized_block_ref().await?;
    let current_block = api.backend().block_header(block_ref.hash()).await?
        .map(|h| h.number)
        .unwrap_or(0);

    let storage = api.storage().at_latest().await?;
    let count = storage.fetch(&polkadot::storage().treasury().spend_count())
        .await?
        .unwrap_or(0);

    let mut result = Vec::new();

    for index in 0..count {
        if let Some(spend) = storage
            .fetch(&polkadot::storage().treasury().spends(index))
            .await?
        {
            let status = format!("{:?}", spend.status);
            if !status.contains("Attempted") &&
               current_block >= spend.valid_from &&
               current_block <= spend.expire_at {
                result.push(Payable {
                    index,
                    amount: spend.amount,
                    beneficiary: format!("{:?}", spend.beneficiary),
                });
            }
        }
    }

    Ok(result)
}

fn display_payables(payables: &[Payable]) {
    if payables.is_empty() {
        println!("No eligible treasury spends found.");
        return;
    }

    println!("Found {} eligible treasury spends:", payables.len());
    for p in payables {
        println!(
            "index={}, amount={}, beneficiary={}",
            p.index, p.amount, p.beneficiary
        );
    }
}

async fn execute_payment(
    api: &OnlineClient<PolkadotConfig>,
    keypair: &Keypair,
    payable: &Payable,
) -> Result<()> {
    let tx = polkadot::tx().treasury().payout(payable.index);
    match api.tx().sign_and_submit_default(&tx, keypair).await {
        Ok(hash) => {
            println!("✓ index={} submitted: {}", payable.index, hash);
            Ok(())
        }
        Err(e) => {
            println!("✗ index={} failed: {}", payable.index, e);
            Err(e.into())
        }
    }
}

async fn execute_payments(
    api: &OnlineClient<PolkadotConfig>,
    keypair: &Keypair,
    payables: &[Payable],
) -> Result<()> {
    println!("→ Sending {} transactions", payables.len());
    let mut ok = 0;

    for p in payables {
        if execute_payment(api, keypair, p).await.is_ok() {
            ok += 1;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("🚀 paypig completed: {}/{} successful", ok, payables.len());
    Ok(())
}
