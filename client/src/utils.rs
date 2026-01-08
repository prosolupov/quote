use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Читает содержимое файла и возвращает его в виде строки.
///
/// Функция открывает файл по указанному пути и полностью
/// считывает его содержимое в память.
///
/// # Аргументы
/// * `path_file` — путь к файлу в строковом виде.
///
/// # Возвращаемое значение
/// * `Ok(String)` — содержимое файла;
/// * `Err(std::io::Error)` — ошибка ввода-вывода
///   (файл не найден, нет прав доступа, ошибка чтения и т.д.).
///
/// # Пример
/// ```no_run
/// let content = read_file("tickers.txt".to_string()).unwrap();
/// println!("{}", content);
/// ```
///
pub fn read_file(path_file: String) -> Result<String, std::io::Error> {
    let path: PathBuf = Path::new(path_file.as_str()).to_owned();
    let mut file: File = File::open(path)?;
    let mut tickers = String::new();
    file.read_to_string(&mut tickers)?;

    Ok(tickers)
}
