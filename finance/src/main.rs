use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use chrono::Utc;
use serde::Deserialize;
use ureq;

// --- Trait Definition ---
trait Pricing {
    fn fetch_price(&mut self) -> Result<(), String>;
    fn save_to_file(&self) -> Result<(), String>;
}

// --- Structs ---
struct Bitcoin {
    price: f64,
}

struct Ethereum {
    price: f64,
}

struct SP500 {
    price: f64,
}

// --- API Response Structs ---
#[derive(Deserialize, Debug)]
struct CombinedPriceResponse {
    bitcoin: CurrencyData,
    ethereum: CurrencyData,
}

#[derive(Deserialize, Debug)]
struct CurrencyData {
    usd: f64,
}

#[derive(Deserialize, Debug)]
struct YahooChart {
    chart: YahooResult,
}

#[derive(Deserialize, Debug)]
struct YahooResult {
    result: Option<Vec<YahooMetaWrapper>>,
}

#[derive(Deserialize, Debug)]
struct YahooMetaWrapper {
    meta: YahooMeta,
}

#[derive(Deserialize, Debug)]
struct YahooMeta {
    regularMarketPrice: f64,
}

// --- Shared JSON Fetching Helper ---
fn get_json<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, String> {
    ureq::get(url)
        .call()
        .map_err(|e| format!("HTTP error: {}", e))?
        .into_json()
        .map_err(|e| format!("JSON parse error: {}", e))
}

// --- Shared Crypto Fetcher ---
fn fetch_crypto_prices() -> Result<(f64, f64), String> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";
    let response: CombinedPriceResponse = get_json(url)?;
    Ok((response.bitcoin.usd, response.ethereum.usd))
}

// --- Implement Pricing Trait ---

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) -> Result<(), String> {
        // This will be handled outside for rate limiting
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), String> {
        save_price_to_file("bitcoin.txt", self.price)
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), String> {
        save_price_to_file("ethereum.txt", self.price)
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        let response: YahooChart = get_json(url)?;
        self.price = response.chart
            .result
            .ok_or("Missing result array")?
            .get(0)
            .ok_or("No data in result")?
            .meta
            .regularMarketPrice;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), String> {
        save_price_to_file("sp500.txt", self.price)
    }
}

// --- Helper Function ---
fn save_price_to_file(filename: &str, price: f64) -> Result<(), String> {
    let timestamp = Utc::now();
    let entry = format!("{},{}\n", timestamp.to_rfc3339(), price);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .map_err(|e| format!("File error: {}", e))?;

    file.write_all(entry.as_bytes())
        .map_err(|e| format!("Write error: {}", e))?;

    Ok(())
}

// --- Main Loop ---
fn main() {
    let mut sp500 = SP500 { price: 0.0 };

    loop {
        println!("--- Fetching prices @ {} ---", Utc::now());

        // --- Fetch Bitcoin & Ethereum in ONE request ---
        match fetch_crypto_prices() {
            Ok((btc_price, eth_price)) => {
                let btc = Bitcoin { price: btc_price };
                let eth = Ethereum { price: eth_price };

                if let Err(e) = btc.save_to_file() {
                    eprintln!("Bitcoin save error: {}", e);
                }

                if let Err(e) = eth.save_to_file() {
                    eprintln!("Ethereum save error: {}", e);
                }

                println!("Bitcoin: ${:.2}, Ethereum: ${:.2}", btc_price, eth_price);
            }
            Err(e) => eprintln!("Crypto fetch error: {}", e),
        }

        // --- Fetch SP500 separately ---
        match sp500.fetch_price() {
            Ok(_) => {
                if let Err(e) = sp500.save_to_file() {
                    eprintln!("SP500 save error: {}", e);
                } else {
                    println!("S&P 500: {:.2}", sp500.price);
                }
            }
            Err(e) => eprintln!("SP500 fetch error: {}", e),
        }

        thread::sleep(Duration::from_secs(10));
    }
}
