use std::env::args;
fn main() {
    let args: Vec<String> = args().skip(1).collect();

    let arg_1 = &args[0];
    println!("argument 1 is {}", arg_1);
}