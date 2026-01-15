use crate::models::{CommandClient, Commands, StockQuote};
use crossbeam::channel::{Receiver, RecvTimeoutError};
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};

/// Создает рабочий поток для обработки команды клиента.
///
/// Функция запускает отдельный поток, который:
/// - создает UDP-сокет для отправки данных;
/// - обрабатывает команду `STREAM`;
/// - запускает производителей котировок для каждого тикера;
/// - принимает `ping`/`pong` сообщения по UDP;
/// - пересылает сгенерированные котировки на указанный endpoint;
/// - завершает работу при отсутствии `ping` или остановке производителей.
pub fn create_worker(command: CommandClient, rx: Receiver<StockQuote>) {
    thread::spawn(move || {
        let sender = UdpSocket::bind("0.0.0.0:0").unwrap();
        sender.set_nonblocking(true).unwrap();

        match command.command {
            Commands::Stream { endpoint, tickets } => {
                let mut buf = [0u8; 256];
                let mut last_ping = Instant::now();
                let ping_timeout = Duration::from_secs(6);

                loop {
                    loop {
                        match sender.recv_from(&mut buf) {
                            Ok((len, src)) => {
                                let msg = &buf[..len];

                                if msg == b"ping" {
                                    last_ping = Instant::now();
                                    sender.send_to(b"pong", src).unwrap();
                                    tracing::info!("Got ping from {}, sent pong", src);
                                }
                            }
                            Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                            Err(e) => {
                                tracing::error!("recv_from error: {}", e);
                                break;
                            }
                        }
                    }

                    if last_ping.elapsed() > ping_timeout {
                        tracing::error!("No ping for {:?}, stop streaming", ping_timeout);
                        break;
                    }

                    match rx.recv_timeout(Duration::from_millis(200)) {
                        Ok(quote) => {
                            if tickets.contains(&quote.ticker) {
                                let msg = format!("{}: {}", quote.ticker, quote.price);

                                sender.send_to(msg.as_bytes(), &endpoint.address).unwrap();

                                println!("Sent {} to {}", quote.ticker, endpoint.address);
                            }
                        }
                        Err(RecvTimeoutError::Timeout) => {}
                        Err(RecvTimeoutError::Disconnected) => {
                            tracing::error!("Generator stopped");
                            break;
                        }
                    }
                }
            }
        }
    });
}
