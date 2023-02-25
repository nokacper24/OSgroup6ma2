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
    let mut ticket_server = TicketServer::new("The Matrix", 15);

    let mut threads = vec![];

    let mut server_refs: Vec<&mut TicketServer> = vec![];
    unsafe {
        for _i in 0..6 {
            let a = &mut *(&mut ticket_server as *mut TicketServer);
            server_refs.push(a);
        }
    }
    // using unwrap since I know there is 6 items and we're already breaking the rules by using unsafe...
    let clients = vec![
        TickerClient::new("Bob", 5, server_refs.pop().unwrap()),
        TickerClient::new("Alice", 3, server_refs.pop().unwrap()),
        TickerClient::new("Jake", 1, server_refs.pop().unwrap()),
        TickerClient::new("Thomas", 5, server_refs.pop().unwrap()),
        TickerClient::new("John", 2, server_refs.pop().unwrap()),
        TickerClient::new("Jane", 3, server_refs.pop().unwrap()),
    ];

    for (i, mut client) in clients.into_iter().enumerate() {
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
