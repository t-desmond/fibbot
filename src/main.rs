use std::env::args;
fn main() {
    let args: Vec<String> = args().skip(1).collect();

    let argument_1 = &args[0];
    let argument_2 = &args[1];
    println!("argument 1 is {}", argument_1);
    println!("argument 1 is {}", argument_2);
}