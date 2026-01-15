mod commands;
mod connector;
mod utils;

use crate::connector::{create_connection_server, create_listener};
use clap::Parser;
use tracing_subscriber::fmt;
use commands::CliCommands;

/// Точка входа в приложение.
///
/// Функция:
/// - разбирает аргументы командной строки;
/// - устанавливает TCP-соединение с сервером и отправляет команду `STREAM`;
/// - запускает UDP-слушатель для приема сообщений.
///
fn main() {
    fmt().with_max_level(tracing::Level::INFO).init();

    let params = CliCommands::parse();

    tracing::info!("{:?}", params);
    let port_udp = params.port_udp.clone();

    if let Err(e) = create_connection_server(params) {
        tracing::info!("Error: {}", e);
    };

    if let Err(e) = create_listener(port_udp) {
        tracing::info!("Error: {}", e);
    };
}
