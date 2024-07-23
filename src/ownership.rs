pub fn move_function() {
    // move
    let s1 = String::from("hello");
    let s2 = s1;
    // move 된 값이라는 오류가 뜸
    println!("{}, world!", s1);
}
