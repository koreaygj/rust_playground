// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

// * Use a HashMap for the furniture store stock
fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);
    let mut total_stock = 0;
    for (name, value) in stock.iter() {
        total_stock += value;
        let stock_count = if *value == 0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", value)
        };
        println!("item = {:?}, stock = {:?} ", name, stock_count);
    }
    println!("total stock = {:?}", total_stock);
}
