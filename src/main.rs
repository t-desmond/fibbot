mod fib;
mod get_numbers;

use fib::Fibbonacci;
use get_numbers::GetNumbers;
use num_bigint::BigInt;
use std::env::{self};
fn main() {
    // let args: Vec<String> = args().skip(1).collect();

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "false".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());
    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    if enable_fib.eq("true") {
        println!("fibbot enabled...");
        let pr_content = "this is a simulated pr with 45 9 25";
        let numbers = GetNumbers::extract_numbers(pr_content);
        let numbers: Vec<BigInt> = numbers
            .into_iter()
            .filter(|x| x < &max_threshold.parse::<u32>().unwrap())
            .map(|x| Fibbonacci::fibbo(x.into()))
            .collect();
        println!("{:?}", numbers);
    } else {
        println!("fibbot disabled...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigInt;
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
    fn test_extract_numbers() {
        assert_eq!(
            GetNumbers::extract_numbers("pr_ d 888 escription 888 67 4b 66"),
            [888, 888, 67, 66]
        );
        assert_eq!(GetNumbers::extract_numbers(" "), []);
    }
}
