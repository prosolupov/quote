mod commands;
mod connector;
mod utils;

use crate::connector::{create_connection_server, create_listener};
use clap::Parser;
use commands::CliCommands;

/// Точка входа в приложение.
///
/// Функция:
/// - разбирает аргументы командной строки;
/// - устанавливает TCP-соединение с сервером и отправляет команду `STREAM`;
/// - запускает UDP-слушатель для приема сообщений.
///
fn main() {
    let params = CliCommands::parse();

    println!("{:?}", params);
    let port_udp = params.port_udp.clone();

    if let Err(e) = create_connection_server(params) {
        println!("Error: {}", e);
    };

    if let Err(e) = create_listener(port_udp) {
        println!("Error: {}", e);
    };
}
