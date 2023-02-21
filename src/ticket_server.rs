use log::{info, error};


/// Represents a ticket server that sells tickets for a movie.
/// The server has a limited amount of seats available.
/// If a client requests more tickets than available, the request is rejected.
/// 
/// The server writes a log message for each request.
pub struct TicketServer {
    moovie_name: String,
    available_seats: u32,
}
impl TicketServer {
    pub fn new(moovie_name: &str, available_seats: u32) -> Self {
        Self {
            moovie_name: moovie_name.to_string(),
            available_seats,
        }
    }
    pub fn book_tickets(&mut self, user_name: &str, amount: u32) -> Result<(), String> {
        info!("Hi, {}! There are {} seats available for {} and you'd like to buy {}.", user_name, self.available_seats, self.moovie_name, amount);

        if self.available_seats >= amount {
            self.available_seats -= amount;
            info!("Enjoy the movie, {}! Remaining seats: {}", user_name, self.available_seats);
            Ok(())
        } else {
            error!("Sorry {}, we have {} seats left.", user_name, self.available_seats);
            Err(format!("Sorry {}, we have only {} seats left", user_name, self.available_seats))
        }
    }
}