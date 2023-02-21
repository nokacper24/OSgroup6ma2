use std::sync::{Arc, Mutex};

use log::error;

use crate::ticket_server::TicketServer;

pub struct TickerClient {
    name: String,
    amount: u32,
    server: Arc<Mutex<TicketServer>>,
}

/// Represents a client that wants to book tickets.
/// Client only sends a request for given amount of tickets,
/// ifnores whether the request was successful or not.
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
                _ = server.book_tickets(&self.name, self.amount);
            }
            Err(e) => {
                error!("Error locking server: {}", e)
            }
        }
    }
}
