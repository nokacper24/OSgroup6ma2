use std::sync::{Mutex, Arc};

use ticket_server::TicketServer;
mod ticket_server;

fn main() {
    let ticket_server = Arc::new(Mutex::new(TicketServer::new("The Matrix", 20)));
    
    let ticket_server1 = Arc::clone(&ticket_server);
        let t1 = std::thread::spawn(move || {
            match ticket_server1.lock(){
                Ok(mut ticket_server) => {
                    if let Err(e) = ticket_server.book_tickets("Alice", 5) {
                        println!("Error: {}", e);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
        }});

        let ticket_server2 = Arc::clone(&ticket_server);
        let t2 = std::thread::spawn(move || {
            match ticket_server2.lock(){
                Ok(mut ticket_server) => {
                    if let Err(e) = ticket_server.book_tickets("Bob", 10) {
                        println!("Error: {}", e);
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
        }});


        t1.join().unwrap();
        t2.join().unwrap();
    


}
