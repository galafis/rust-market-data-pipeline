use polars::prelude::*;
use reqwest;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct AlphaVantageResponse {
    #[serde(rename = "Time Series (Daily)")]
    pub time_series: std::collections::HashMap<String, DailyData>,
}

#[derive(Debug, Deserialize)]
pub struct DailyData {
    #[serde(rename = "1. open")]
    pub open: String,
    #[serde(rename = "2. high")]
    pub high: String,
    #[serde(rename = "3. low")]
    pub low: String,
    #[serde(rename = "4. close")]
    pub close: String,
    #[serde(rename = "5. volume")]
    pub volume: String,
}

pub async fn fetch_alphavantage_data(symbol: &str, api_key: &str) -> Result<DataFrame> {
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}",
        symbol,
        api_key
    );

    let response = reqwest::get(&url).await?.json::<AlphaVantageResponse>().await?;

    let mut dates = Vec::new();
    let mut opens = Vec::new();
    let mut highs = Vec::new();
    let mut lows = Vec::new();
    let mut closes = Vec::new();
    let mut volumes = Vec::new();

    for (date, data) in response.time_series {
        dates.push(date);
        opens.push(data.open.parse::<f64>()?);
        highs.push(data.high.parse::<f64>()?);
        lows.push(data.low.parse::<f64>()?);
        closes.push(data.close.parse::<f64>()?);
        volumes.push(data.volume.parse::<i64>()?);
    }

    let df = DataFrame::new(vec![
        Series::new("date", dates),
        Series::new("open", opens),
        Series::new("high", highs),
        Series::new("low", lows),
        Series::new("close", closes),
        Series::new("volume", volumes),
    ])?;

    Ok(df)
}

