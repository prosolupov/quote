use crate::generat_data::QuoteGenerator;
use crate::models::StockQuote;
use crossbeam::channel::Sender;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// Генерирует новую котировку для указанного тикера.
///
/// Возвращает `Some(StockQuote)` при успешной генерации
/// или `None`, если генерация невозможна.
fn create_generate(ticker: &str) -> Option<StockQuote> {
    let mut generate = QuoteGenerator::new();
    let ticket: Option<StockQuote> = generate.generate_quote(ticker);
    ticket
}

/// Запускает производителя котировок в отдельном потоке.
///
/// Функция периодически генерирует котировки для заданного тикера
/// и отправляет их через канал. Работа потока завершается,
/// если получатель канала закрыт или генерация невозможна.
pub fn producer(ticker: String, tx: Sender<StockQuote>) {
    thread::spawn(move || {
        loop {
            if let Some(q) = create_generate(&ticker) {
                if tx.send(q).is_err() {
                    break;
                }
            } else {
                eprintln!("No ticker specified");
                break;
            }
            sleep(Duration::from_secs(5));
        }
    });
}
