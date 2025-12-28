use crate::generat_data::QuoteGenerator;
use crate::models::{CommandClient, StockQuote};
use crossbeam::channel;
use crossbeam::channel::Sender;
use std::collections::hash_map::Entry::Vacant;
use std::io::{BufRead, Read, Write};
use std::net::{Shutdown, TcpListener, UdpSocket};
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod generat_data;
mod models;

fn create_generate(ticker: &str) -> Option<StockQuote> {
    let mut generate = QuoteGenerator::new();
    let ticket: Option<StockQuote> = generate.generate_quote(ticker);
    ticket
}

fn create_producer(tickers: Vec<String>) -> crossbeam::channel::Receiver<StockQuote> {
    let (tx, rx) = channel::unbounded::<StockQuote>();
    // let ticket = create_generate(ticker);

    let args = |tickers: Vec<String>| {
        tickers
            .iter()
            .filter_map(|t| create_generate(t))
            .collect::<Vec<StockQuote>>()
    };

    thread::spawn(move || {
       for arg in args(tickers) {
           tx.send(arg).unwrap();
       }
    });

    rx
}

fn create_thread(command: CommandClient) {
    thread::spawn(move || {
        let sender = UdpSocket::bind("0.0.0.0:0").unwrap();
        println!("{}", command.tickers[0]);
        println!("Command {}", command.schema);

        let rx = create_producer(command.tickers);

        for data in rx {

            for i in 0..10 {
                sleep(Duration::from_secs(10));
                sender
                    .send_to(data.to_string().as_bytes(), &command.address)
                    .unwrap();

                println!(
                    "Sent data {} to IP {}, ID_process {:?}",
                    i,
                    command.address,
                    thread::current().id()
                );
            }
        }
    });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let ticket = create_generate("ticker");

    loop {
        let mut buffer = String::new();
        let (mut stream, _) = listener.accept()?;
        stream.read_to_string(&mut buffer)?;
        println!("Ip: {}", buffer);

        match CommandClient::parse_command(&*buffer.to_string()) {
            Ok(command) => {
                create_thread(command);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    Ok(())
}
