// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

// * Use a struct to store at least the age of a customer
// * Determine if a customer is able to make a restricted purchase
#[derive(Debug)]
struct Customer {
    name: String,
    age: i32,
}
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
fn check_age(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err("customer must be at least 21 years old...".to_owned())
    }
}
fn main() {
    let customer = Customer {
        name: "yang".to_owned(),
        age: 20,
    };
    let result = check_age(&customer);
    print!("{:?}", result);
}
