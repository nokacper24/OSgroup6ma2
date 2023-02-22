use std::sync::{Arc, Mutex};

use log::{error, info};
use ticket_client::TickerClient;
use ticket_server::TicketServer;
mod ticket_client;
mod ticket_server;
use std::io::Write;

use log::LevelFilter;

fn main() {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "Thread: {} [{}] - {}",
                std::thread::current().name().unwrap_or("unnamed"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    info!("Starting the ticket server...");
    let ticket_server = Arc::new(Mutex::new(TicketServer::new("The Matrix", 15)));
    let mut threads = vec![];

    let clients = vec![
        TickerClient::new("Bob", 5, Arc::clone(&ticket_server)),
        TickerClient::new("Alice", 3, Arc::clone(&ticket_server)),
        TickerClient::new("Jake", 1, Arc::clone(&ticket_server)),
        TickerClient::new("Thomas", 5, Arc::clone(&ticket_server)),
        TickerClient::new("John", 2, Arc::clone(&ticket_server)),
        TickerClient::new("Jane", 3, Arc::clone(&ticket_server)),
    ];

    for (i, client) in clients.into_iter().enumerate() {
        let t = std::thread::Builder::new()
            .name(format!("T{}", i + 1))
            .spawn(move || {
                client.book_tickets();
            });
        threads.push(t);
    }

    for t in threads.into_iter().flatten() {
        if t.join().is_err() {
            error!("Error joining thread");
        }
    }
}
