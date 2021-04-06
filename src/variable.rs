pub fn run() {
    let value = 7; // by default variables are immutable
    let mut number = 3; // mutable variable
    println!("value assigned: {}", value);
    println!("number variable assigned : {}", number);
    number = 9;
    println!("number variable changed to {}", number);
}
