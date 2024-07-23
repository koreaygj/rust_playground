mod ownership;

fn main() {
    ownership::move_function();
    ownership::clone_string();
    ownership::copy_stack();

    let s = String::from("hello"); // s가 스코프 안으로 들어옵니다

    ownership::takes_ownership(s); // s의 값이 함수로 이동됩니다...

    // ... 따라서 여기서는 더 이상 유효하지 않습니다
    // println!("{}", s);

    let x = 5; // x가 스코프 안으로 들어옵니다
    ownership::makes_copy(x); // x가 함수로 이동될 것입니다만,

    // i32는 Copy이므로 앞으로 계속 x를
    println!("{}", x);
}
