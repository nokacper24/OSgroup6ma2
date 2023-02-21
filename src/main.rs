use std::{sync::{Arc, Mutex}, fmt::format};

use log::{info, error};
use ticket_client::TickerClient;
use ticket_server::TicketServer;
mod ticket_client;
mod ticket_server;
use std::io::Write;

use log::LevelFilter;

fn main() {
    env_logger::Builder::new()
    .format(|buf, record| {
        writeln!(buf,
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

    let mut i = 1;
    for client in clients {
        let t = std::thread::Builder::new().name(format!("T{}",i)).spawn(move || {
            client.book_tickets();
        });
        threads.push(t);
        i += 1;
    }

    for t in threads {
        if let Ok(t) = t {
            if let Err(_) = t.join() {
                error!("Error joining thread");
            }
        }
    }
}
