// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    // * The color and name should be stored as a String
    name: String,
    favorite_color: String,
}

fn print_name(name: &str) {
    println!("name : {:?}", name);
}
fn print_favorite_color(color: &str) {
    println!("favorite color: {:?}", color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 20,
            name: "yang".to_owned(),
            favorite_color: "yellow".to_owned(),
            // or use String::from
            // name: String::from("yang");
            // favorite_color: String::from("yellow");
        },
        Person {
            age: 24,
            name: "kim".to_owned(),
            favorite_color: "blue".to_owned(),
        },
        Person {
            age: 10,
            name: "song".to_owned(),
            favorite_color: "black".to_owned(),
        },
    ];
    // * Iterate through the vector using a for..in loop
    for man in people {
        // * Use an if expression to determine which person's info should be printed
        if man.age >= 10 {
            // * The name and colors should be printed using a function
            print_name(&man.name);
            print_favorite_color(&man.favorite_color);
        }
    }
}
