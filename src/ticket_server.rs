
#[derive(Debug, Clone)]
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
        println!("Hi, {}! There are {} seats available for {}.", user_name, self.available_seats, self.moovie_name);

        if self.available_seats >= amount {
            self.available_seats -= amount;
            println!("Enjoy the movie, {}! There are still {} seats left.", user_name, self.available_seats);
            Ok(())
        } else {
            println!("Sorry {}, we have only {} seats left", user_name, self.available_seats);
            Err(format!("Sorry {}, we have only {} seats left", user_name, self.available_seats))
        }
    }
}