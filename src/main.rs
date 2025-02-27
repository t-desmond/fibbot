mod get_numbers;
mod fib;

use std::env::{self, args};
fn main() {
    let args: Vec<String> = args().skip(1).collect();

    // let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap();
    // let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap();
    // println!("Enable Fibonacci Calculation: {}", enable_fib);
    // println!("Max Threshold: {}", max_threshold);
    if args.is_empty(){
        println!("no arguments supplied");
        return;
    } else if args.len() == 1 {
        println!("Fibbot requires two parameters");
    } else if args.len() ==2  {
        let argument_2 = &args[1];
        println!("fiboot enabled succesfully with max_threshold: {}", argument_2);
    }
}