#![warn(missing_docs)]

use clap::{Parser, Subcommand};

/// Биржевая котировка акции.
///
/// Содержит базовую информацию о торговом событии:
/// - тикер инструмента,
/// - цену,
/// - объем сделки,
/// - временную метку (timestamp).
#[derive(Debug, Clone)]
pub struct StockQuote {
    /// Тикер акции (например: AAPL, TSLA).
    pub ticker: String,
    /// Цена одной акции.
    pub price: f64,
    /// Объем сделки.
    pub volume: u32,
    /// Временная метка в формате Unix timestamp.
    pub timestamp: u64,
}

impl StockQuote {
    /// Сериализует котировку в строку.
    ///
    /// Формат:
    /// `TICKER|PRICE|VOLUME|TIMESTAMP`
    ///
    /// # Пример
    /// ```
    /// let quote = StockQuote {
    ///     ticker: "AAPL".into(),
    ///     price: 190.5,
    ///     volume: 100,
    ///     timestamp: 1710000000,
    /// };
    /// assert_eq!(quote.to_string(), "AAPL|190.5|100|1710000000");
    /// ```

    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.ticker, self.price, self.volume, self.timestamp
        )
    }

    /// Десериализует котировку из строки.
    ///
    /// Ожидаемый формат:
    /// `TICKER|PRICE|VOLUME|TIMESTAMP`
    ///
    /// Возвращает `None`, если строка имеет неверный формат
    /// или одно из полей не удалось распарсить.
    #[allow(dead_code)]
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split("|").collect();
        if parts.len() == 4 {
            Some(StockQuote {
                ticker: parts[0].to_string(),
                price: parts[1].parse().ok()?,
                volume: parts[2].parse().ok()?,
                timestamp: parts[3].parse().ok()?,
            })
        } else {
            None
        }
    }

    /// Сериализует котировку в байтовый массив.
    ///
    /// Формат аналогичен строковой сериализации,
    /// но возвращается `Vec<u8>`.
    ///
    /// Может использоваться для отправки по сети.
    ///
    /// ⚠️ Не является бинарным протоколом —
    /// это текстовое представление в байтах.
    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.ticker.as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.price.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.volume.to_string().as_bytes());
        bytes.push(b'|');
        bytes.extend_from_slice(self.timestamp.to_string().as_bytes());
        bytes
    }
}

/// Корневая структура CLI-клиента.
///
/// Используется `clap` для разбора аргументов командной строки.
/// Ожидает обязательную подкоманду.
#[derive(Parser, Debug)]
#[command(
    no_binary_name = true,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct CommandClient {
    #[command(subcommand)]
    pub command: Commands,
}

/// Описание сетевого endpoint.
///
/// Представляет собой разобранную строку вида:
/// `<protocol>://<address>`
#[derive(Parser, Debug, Clone)]
pub struct Endpoint {
    pub protocol: String,
    pub address: String,
}

/// Парсит endpoint из строки.
///
/// Ожидаемый формат:
/// `<protocol>://<address>`
///
/// # Ошибки
/// Возвращает `Err`, если строка не содержит `://`.
///
/// # Пример
/// ```
/// let ep = parse_endpoint("tcp://127.0.0.1:9000").unwrap();
/// assert_eq!(ep.protocol, "tcp");
/// assert_eq!(ep.address, "127.0.0.1:9000");
/// ```
fn parse_endpoint(s: &str) -> Result<Endpoint, String> {
    let (protocol, address) = s
        .split_once("://")
        .ok_or("endpoint must be in format <proto>://<address>")?;

    Ok(Endpoint {
        protocol: protocol.to_string(),
        address: address.to_string(),
    })
}

/// Поддерживаемые команды CLI.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Запуск стриминга котировок.
    ///
    /// Принимает endpoint и список тикеров.
    ///
    /// Поддерживаемые алиасы команды:
    /// - stream
    /// - Stream
    /// - STREAM
    #[command(name = "stream", alias = "Stream", alias = "STREAM")]
    Stream {
        /// Endpoint в формате `<protocol>://<address>`.
        #[arg(value_parser = parse_endpoint)]
        endpoint: Endpoint,
        /// Список тикеров, разделенных запятой.
        ///
        /// Пример:
        /// `AAPL,TSLA,GOOG`
        #[arg(value_delimiter = ',')]
        tickets: Vec<String>,
    },
}
