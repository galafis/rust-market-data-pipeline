use rmdp_core::process_and_store_data;
use rmdp_data::fetch_alphavantage_data;
use rmdp_utils::setup_logger;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger();

    // Substitua "YOUR_API_KEY" pela sua chave de API da Alpha Vantage
    let api_key = "YOUR_API_KEY";
    let symbol = "IBM";
    let output_path = "data/ibm_daily.parquet";

    let df = fetch_alphavantage_data(symbol, api_key).await?;
    println!("Dados brutos recebidos:\n{}", df);

    process_and_store_data(df, output_path)?;
    println!("Dados processados e armazenados em {}", output_path);

    Ok(())
}

