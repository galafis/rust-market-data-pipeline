use polars::prelude::*;
use std::fs::File;
use anyhow::Result;

pub fn process_and_store_data(mut df: DataFrame, path: &str) -> Result<()> {
    // Exemplo de processamento: ordenar por data
    df = df.sort(["date"], false, false)?;

    // Armazenar em formato Parquet
    let mut file = File::create(path)?;
    ParquetWriter::new(&mut file).finish(&mut df)?;

    Ok(())
}
