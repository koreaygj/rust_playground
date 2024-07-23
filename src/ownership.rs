pub fn move_function() {
    // move
    let s1 = String::from("hello");
    let s2 = s1;
    // move 된 값이라는 오류가 뜸
    println!("{}, world!", s2);
}

pub fn clone_string() {
    // clone
    let s1: String = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
