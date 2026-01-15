use crate::models::{CommandClient, StockQuote};
use crate::producer::producer;
use crate::worker::create_worker;
use clap::Parser;
use crossbeam::channel::bounded;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use tracing_subscriber::{fmt, EnvFilter};

mod generat_data;
mod models;
mod producer;
mod worker;

/// Точка входа в сервер.
///
/// Архитектура:
/// 1. Создаётся один общий канал котировок.
/// 2. В отдельном потоке запускается единый генератор (`producer`).
/// 3. Сервер принимает TCP-команды клиентов.
/// 4. Для каждого клиента создаётся worker-поток,
///    который получает клон общего receiver и фильтрует котировки.
fn main() -> Result<(), Box<dyn Error>> {
    fmt().with_max_level(tracing::Level::INFO).init();

    // let (tx, rx) = bounded::<StockQuote>(1000);
    let (tx, rx) = bounded::<StockQuote>(1000);
    producer(tx);

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream?;
        let mut reader = BufReader::new(stream);
        let mut line = String::new();

        reader.read_line(&mut line)?;

        let command = CommandClient::parse_from(line.split_whitespace());

        let rx_clone = rx.clone();

        create_worker(command, rx_clone);
    }

    Ok(())
}
