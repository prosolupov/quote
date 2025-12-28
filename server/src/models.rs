use url::Url;

#[derive(Debug, Clone)]
pub struct StockQuote {
    pub ticker: String,
    pub price: f64,
    pub volume: u32,
    pub timestamp: u64,
}

impl StockQuote {
    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.ticker, self.price, self.volume, self.timestamp
        )
    }

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

    // Или бинарная сериализация
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

#[derive(Debug, Clone)]
pub struct CommandClient {
    pub schema: String,
    pub address: String,
    pub tickers: Vec<String>,
}

impl CommandClient {
    pub fn parse_command(command: &str) -> Result<CommandClient, String> {
        let command = command.trim();

        let mut parts = command.splitn(2, " ");
        let url_parts = parts
            .next()
            .ok_or(format!("Invalid command: {}", command))?;
        let args_parts = parts
            .next()
            .ok_or(format!("Invalid command: {}", command))?;

        let url = Url::parse(url_parts).map_err(|_| format!("Invalid url: {}", url_parts))?;
        let schema = url.scheme().to_string();

        let host = url.host_str().ok_or(format!("Invalid host: {}", url))?;
        let port = url
            .port_or_known_default()
            .ok_or(format!("Invalid port: {}", url))?;

        let address = format!("{}:{}", host, port);

        let tickers = args_parts
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if tickers.is_empty() {
            return Err(format!("no tickers provided: {}", command));
        }

        Ok(CommandClient {
            schema,
            address,
            tickers,
        })
    }
}
