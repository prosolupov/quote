use crate::models::CommandClient;
use crate::worker::create_worker;
use clap::Parser;
use std::io::{BufRead, BufReader};
use std::net::TcpListener;

mod generat_data;
mod models;
mod producer;
mod worker;

/// Точка входа в TCP-сервер.
///
/// Функция:
/// - привязывается к адресу `127.0.0.1:8080`;
/// - принимает входящие TCP-соединения;
/// - читает одну строку команды от клиента;
/// - парсит команду с помощью `clap`;
/// - передает управление рабочему обработчику.
///
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream?;
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let commands = CommandClient::parse_from(line.split_whitespace());

        create_worker(commands);
    }

    Ok(())
}
