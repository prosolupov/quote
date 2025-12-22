use crate::generat_data::QuoteGenerator;
use crate::models::StockQuote;
use std::io::{BufRead, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

mod generat_data;
mod models;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let mut generate = QuoteGenerator::new();
    let ticket = generate.generate_quote("AAPL");
    let (tx, rx) = mpsc::channel::<StockQuote>();

    thread::spawn(move || {
        tx.send(ticket.unwrap())
    });

    let msg = rx.recv()?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Connection established");
        stream.write_all(msg.to_string().as_bytes())?;

    }

    Ok(())
}