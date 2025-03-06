// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables

// * Use a function that returns a tuple
// * Destructure the return value into two variables
fn coordinate() -> (i32, i32) {
    (1, 7)
}
// * Use an if..else if..else block to determine what to print

fn main() {
    let (x, y) = coordinate();
    if y > 5 {
        println!("{:?}, {:?} y value is greater than 5", x, y);
    } else if y == 5 {
        println!("{:?}, {:?} y value is equal 5", x, y);
    } else {
        println!("{:?}, {:?} y value is less 5", x, y);
    }
}
