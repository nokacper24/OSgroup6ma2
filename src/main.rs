use std::sync::{Arc, Mutex};

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
    let ticket_server = Arc::new(Mutex::new(TicketServer::new("The Matrix", 12)));
    let mut threads = vec![];

    let client1 = TickerClient::new("Bob", 5, Arc::clone(&ticket_server));
    let t1 = std::thread::Builder::new().name("T1".to_string()).spawn(move || {

        client1.book_tickets();
    });
    threads.push(t1);

    let client2 = TickerClient::new("Alice", 5, Arc::clone(&ticket_server));
    let t2 = std::thread::Builder::new().name("T2".to_string()).spawn(move || {
        client2.book_tickets();
    });
    threads.push(t2);

    let client3 = TickerClient::new("Jake", 1, Arc::clone(&ticket_server));
    let t3 = std::thread::Builder::new().name("T3".to_string()).spawn(move || {
        client3.book_tickets();
    });
    threads.push(t3);

    let client4 = TickerClient::new("Thomas", 5, Arc::clone(&ticket_server));
    let t4 = std::thread::Builder::new().name("T4".to_string()).spawn(move || {
        client4.book_tickets();
    });
    threads.push(t4);



    for t in threads {
        if let Ok(t) = t {
            if let Err(_) = t.join() {
                error!("Error joining thread");
            }
        }
    }
}
