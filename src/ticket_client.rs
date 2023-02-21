use std::sync::{Mutex, Arc};

use crate::ticket_server::TicketServer;

pub struct TickerClient {
    name: String,
    amount: u32,
    server: Arc<Mutex<TicketServer>>,
}

impl TickerClient {
    pub fn new(name: &str, amount: u32, server: Arc<Mutex<TicketServer>>) -> Self {
        Self {
            name: name.to_string(),
            amount,
            server,
        }
    }
    pub fn book_tickets(&self) -> () {
        match self.server.lock() {
            Ok(mut server) => {
                if let Err(e) = server.book_tickets(&self.name, self.amount) {
                    println!("Error: {}", e);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}