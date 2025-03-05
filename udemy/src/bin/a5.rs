// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    // * Use a mutable integer variable
    let mut i = 1;
    // * Use a loop statement
    loop {
        println!("{:?}", i);
        i += 1;
        // * Use break to exit the loop
        if i == 5 {
            break;
        }
    }
}
