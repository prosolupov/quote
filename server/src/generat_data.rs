use crate::models::StockQuote;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct QuoteGenerator {
    last_prices: HashMap<String, f64>,
}

impl QuoteGenerator {
    pub fn new() -> Self {
        Self {
            last_prices: HashMap::new(),
        }
    }

    pub fn generate_quote(&mut self, ticker: &str) -> Option<StockQuote> {
        // ... логика изменения цены ...

        let last_price = self.last_prices.entry(ticker.to_string()).or_insert(100.0); // стартовая цена

        // 🔹 изменение цены (random walk)
        let delta = rand::random::<f64>() * 2.0 - 1.0; // [-1; 1)
        *last_price = (*last_price + delta).max(1.0); // цена не может быть < 1

        let volume = match ticker {
            // Популярные акции имеют больший объём
            "AAPL" | "MSFT" | "TSLA" => 1000 + (rand::random::<f64>() * 5000.0) as u32,
            // Обычные акции - средний объём
            _ => 100 + (rand::random::<f64>() * 1000.0) as u32,
        };

        Some(StockQuote {
            ticker: ticker.to_string(),
            price: *last_price,
            volume,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        })
    }
}
