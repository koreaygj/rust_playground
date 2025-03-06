// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}
// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(name: Color) {
    match name {
        Color::Red => println!("RED"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow"),
    }
}
fn main() {
    print_color(Color::Red);
    print_color(Color::Blue);
    print_color(Color::Green);
    print_color(Color::Yellow);
}
