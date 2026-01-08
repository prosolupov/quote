use crate::models::StockQuote;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Генератор псевдослучайных биржевых котировок.
///
/// Хранит последнее значение цены для каждого тикера
/// и использует его для генерации следующей котировки.
 pub struct QuoteGenerator {
    /// Последняя сгенерированная цена по каждому тикеру
    last_prices: HashMap<String, f64>,
}

impl QuoteGenerator {
    /// Создает новый генератор котировок.
    ///
    /// Начальные цены будут инициализированы при первой генерации.
    pub fn new() -> Self {
        Self {
            last_prices: HashMap::new(),
        }
    }

    /// Генерирует новую котировку для указанного тикера.
    ///
    /// Цена изменяется на случайное значение в диапазоне `[-1; 1)`,
    /// но не может опуститься ниже `1.0`.
    /// Объем зависит от популярности тикера.
    pub fn generate_quote(&mut self, ticker: &str) -> Option<StockQuote> {
        let last_price = self.last_prices.entry(ticker.to_string()).or_insert(100.0);

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
                .ok()?
                .as_millis() as u64,
        })
    }
}
