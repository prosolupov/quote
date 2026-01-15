use crate::commands::CliCommands;
use crate::utils::read_file;
use std::io::Write;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Создает UDP-слушатель на указанном порту и обрабатывает входящие сообщения.
///
/// Функция:
/// - привязывает неблокирующий UDP-сокет к `127.0.0.1:<port>`;
/// - запоминает адрес последнего отправителя;
/// - периодически отправляет `ping` последнему источнику в отдельном потоке;
/// - выводит полученные сообщения в stdout.
///
/// # Аргументы
/// * `port` — UDP-порт для прослушивания.
///
/// # Ошибки
/// Возвращает ошибку, если не удалось привязать UDP-сокет.
///
/// # Примечание
/// Функция выполняется в бесконечном цикле и предназначена для работы
/// как фоновый сетевой слушатель.
pub fn create_listener(port: String) -> Result<(), std::io::Error> {
    let addr = format!("127.0.0.1:{}", port);
    let socket = Arc::new(UdpSocket::bind(addr)?);
    socket.set_nonblocking(true).unwrap();

    let last_src: Arc<Mutex<Option<SocketAddr>>> = Arc::new(Mutex::new(None));

    {
        let socket = socket.clone();
        let last_src = Arc::clone(&last_src);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(2));

                let addr = match last_src.lock() {
                    Ok(guard) => *guard,
                    Err(poisoned) => *poisoned.into_inner(),
                };

                if let Some(addr) = addr {
                    let _ = socket.send_to(b"ping", addr);
                    tracing::info!("Sent ping to {}", addr);
                }
            }
        });
    }

    let mut buf = [0u8; 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, src)) => {
                *last_src.lock().unwrap() = Some(src);

                tracing::info!(
                    "Received {} bytes from {}: {}",
                    len,
                    src,
                    String::from_utf8_lossy(&buf[..len])
                );
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_secs(3));
            }
            Err(e) => {
                tracing::error!("UDP error: {}", e);
            }
        }
    }
}

/// Создает TCP-соединение с сервером и отправляет команду на запуск стриминга.
///
/// Функция:
/// 1. Формирует адрес сервера из IP и TCP-порта;
/// 2. Устанавливает TCP-соединение;
/// 3. Читает список тикеров из файла;
/// 4. Преобразует тикеры в формат, разделенный запятыми;
/// 5. Отправляет серверу команду `STREAM` с UDP-endpoint и списком тикеров.
///
pub fn create_connection_server(commands: CliCommands) -> Result<(), std::io::Error> {
    let host = format!("{}:{}", commands.ip_server, commands.port_server);
    let mut stream = TcpStream::connect(host)?;

    let mut tickers = String::new();

    match read_file(commands.path_file) {
        Ok(result) => {
            tickers = result.replace('\n', ",");
        }
        Err(error) => {
            tracing::error!("{}", error);
        }
    };

    let commands_to_server = format!(
        "STREAM udp://{}:{} {}",
        commands.ip_server,
        commands.port_udp.clone(),
        tickers
    );

    stream
        .write_all(commands_to_server.as_bytes())
        .expect("Couldn't write to server.");

    Ok(())
}
