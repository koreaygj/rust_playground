// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Lockers use numbers and are optional for students
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Locker {
    name: String,
    locker_num: Option<i32>,
}

fn main() {
    let locker = Locker {
        name: "yang".to_owned(),
        locker_num: None,
    };
    // * Print out the details of a student's locker assignment
    println!("student name {:?}", locker.name);
    match locker.locker_num {
        Some(number) => println!("locker number: {:?}", number),
        None => println!("no locker assigned"),
    }
}
