// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let string = "try rust standard library documentation";
    let upper_str = string.to_uppercase();
    println!("upper case {:?}", upper_str);
    let lower_str = upper_str.to_lowercase();
    println!("lower case {:?}", lower_str);
}
