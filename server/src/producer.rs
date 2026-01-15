use crate::generat_data::QuoteGenerator;
use crate::models::StockQuote;
use crossbeam::channel::Sender;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// Запускает единый генератор биржевых котировок.
///
/// Функция:
/// - загружает список тикеров из файла,
/// - создаёт один экземпляр `QuoteGenerator`,
/// - в бесконечном цикле генерирует котировки по всем тикерам,
/// - отправляет каждую котировку в переданный канал `tx`.
///
/// Генерация выполняется в цикле с задержкой 1 секунда.
/// Работа функции завершается с ошибкой, если:
/// - не удаётся прочитать файл с тикерами, или
/// - получатель канала закрыт.
fn create_generate(tx: Sender<StockQuote>) -> Result<(), Box<dyn Error>> {
    let file: File = File::open("server/static/list_tickers.txt")?;
    let reader = BufReader::new(file);
    let mut tickers = reader.lines().collect::<Result<Vec<_>, _>>()?;

    let mut generate = QuoteGenerator::new();

    loop {
        for ticker in tickers.iter_mut() {
            if let Some(tick) = generate.generate_quote(ticker) {
                tx.send(tick)?;
            };
        }
        sleep(Duration::from_secs(1));
    }
}

/// Запускает единый генератор котировок в отдельном потоке.
///
/// Функция создаёт фоновый поток, в котором выполняется `create_generate`.
/// Основной поток при этом не блокируется.
///
/// Если генерация завершается с ошибкой (например, закрыт канал
/// или недоступен файл с тикерами), ошибка логируется через `tracing::error`
pub fn producer(tx: Sender<StockQuote>) {
    thread::spawn(move || {
        if let Err(e) = create_generate(tx) {
            tracing::error!("Generator stopped: {}", e);
        }
    });
}
