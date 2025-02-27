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

#[cfg(test)]
mod tests { 
    use super::*;
    use fib::Fibbonacci;
    use num_bigint::ToBigInt;
    use get_numbers::GetNumbers;

    #[test]
    fn test_fib() {
        let num_1 = 30.to_bigint().unwrap();
        let fib_num_1 = 832040.to_bigint().unwrap();
        assert_eq!(Fibbonacci::fibbo(num_1), fib_num_1);
        
        let num_2 = 50.to_bigint().unwrap();
        let fib_num_2: u64 = 12586269025;
        let fib_num_2 = fib_num_2.to_bigint().unwrap();
        assert_eq!(Fibbonacci::fibbo(num_2), fib_num_2);
        
        let num_3 = 100.to_bigint().unwrap();
        let fib_num_3: u128 = 354224848179261915075;
        let fib_num_3 = fib_num_3.to_bigint().unwrap();
        assert_eq!(Fibbonacci::fibbo(num_3), fib_num_3);
    }
    
    #[test]
    fn test_extract_numbers(){
        assert_eq!(GetNumbers::extract_numbers("pr_ d 888 escription 888 67 4b 66"), [888, 888, 67, 66]);
        assert_eq!(GetNumbers::extract_numbers(" "), [ ]);
    }
}
