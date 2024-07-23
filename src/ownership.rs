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

// 스택에 저장되는 데이터는 복사됨
// 모든 정수형 타입, bool, 모든 부동소수점 타입, 문자 타입
// copy 가능한 타입만으로 이루어진 튜플 ex) (i32, i32)
pub fn copy_stack() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
