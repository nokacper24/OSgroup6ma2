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
    let mut ticket_server = TicketServer::new("The Matrix", 15);
    let mut threads = vec![];

    let server1: &'static mut TicketServer;
    let server2: &'static mut TicketServer;
    let server3: &'static mut TicketServer;
    let server4: &'static mut TicketServer;
    let server5: &'static mut TicketServer;
    let server6: &'static mut TicketServer;
    unsafe {
        server1 = &mut *(&mut ticket_server as *mut TicketServer);
        server2 = &mut *(&mut ticket_server as *mut TicketServer);
        server3 = &mut *(&mut ticket_server as *mut TicketServer);
        server4 = &mut *(&mut ticket_server as *mut TicketServer);
        server5 = &mut *(&mut ticket_server as *mut TicketServer);
        server6 = &mut *(&mut ticket_server as *mut TicketServer);
    }
 
    let clients = vec![
        TickerClient::new("Bob", 5, server1),
        TickerClient::new("Alice", 3, server2),
        TickerClient::new("Jake", 1, server3),
        TickerClient::new("Thomas", 5, server4),
        TickerClient::new("John", 2, server5),
        TickerClient::new("Jane", 3, server6),
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
