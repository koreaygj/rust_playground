// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavor {
    Cherry,
    Watermelon,
    Apple,
    Orange,
}
// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}
// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Apple => println!("Apple , {:?}", drink.fluid_oz),
        Flavor::Watermelon => println!("Watermelon , {:?}", drink.fluid_oz),
        Flavor::Orange => println!("Orange , {:?}", drink.fluid_oz),
        Flavor::Cherry => println!("Cherry , {:?}", drink.fluid_oz),
    }
}
fn main() {
    let drink = Drink {
        flavor: Flavor::Apple,
        fluid_oz: 8.0,
    };
    print_drink(drink);
}
