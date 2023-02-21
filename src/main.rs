use std::sync::{Arc, Mutex};

use ticket_client::TickerClient;
use ticket_server::TicketServer;
mod ticket_client;
mod ticket_server;

fn main() {
    let ticket_server = Arc::new(Mutex::new(TicketServer::new("The Matrix", 12)));
    let mut threads = vec![];

    let client1 = TickerClient::new("Bob", 5, Arc::clone(&ticket_server));
    let t1 = std::thread::spawn(move || {
        client1.book_tickets();
    });
    threads.push(t1);

    let client2 = TickerClient::new("Alice", 5, Arc::clone(&ticket_server));
    let t2 = std::thread::spawn(move || {
        client2.book_tickets();
    });
    threads.push(t2);

    let client3 = TickerClient::new("Jake", 5, Arc::clone(&ticket_server));
    let t3 = std::thread::spawn(move || {
        client3.book_tickets();
    });
    threads.push(t3);

    let client4 = TickerClient::new("Thomas", 5, Arc::clone(&ticket_server));
    let t4 = std::thread::spawn(move || {
        client4.book_tickets();
    });
    threads.push(t4);

    for t in threads {
        if let Err(_) = t.join() {
            println!("Error while joining thread");
        }
    }
}
