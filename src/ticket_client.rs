use std::sync::{Arc, Mutex};

use log::error;

use crate::ticket_server::TicketServer;

pub struct TickerClient<'a> {
    name: String,
    amount: u32,
    server: &'a mut TicketServer,
}

/// Represents a client that wants to book tickets.
/// Client only sends a request for given amount of tickets,
/// ifnores whether the request was successful or not.
impl TickerClient<'_> {
    pub fn new(name: &str, amount: u32, server: &'static mut TicketServer) -> Self {
        Self {
            name: name.to_string(),
            amount,
            server,
        }
    }
    pub fn book_tickets(&mut self) {
        self.server.book_tickets(&self.name, self.amount);
    }
}
