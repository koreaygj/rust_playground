// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}
fn main() {
    // * Create one of each ticket and place into a vector
    let mut my_tickets = vec![];
    my_tickets.push(Ticket::Backstage(100.0, "yang".to_owned()));
    my_tickets.push(Ticket::Vip(300.0, "king".to_owned()));
    my_tickets.push(Ticket::Standard(20.0));
    for i in my_tickets {
        // * Use a match expression while iterating the vector to print the ticket info
        match i {
            Ticket::Backstage(price, holder) => println!("Backstage ticket price:{:?}, holder:{:?} ", price, holder)
            Ticket::Vip(price, holder) => println!("VIP ticket price:{:?}, holder:{:?} ", price, holder)
            Ticket::Standard(price) => println!("Standard ticket price:{:?} ", price),
        }
    }
}
