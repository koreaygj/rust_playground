fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in inner scope is : {x}")
    }
    println!("The value of x is : {x}");
    let guess:u32 = "42".parse().expect("not a number!");
}