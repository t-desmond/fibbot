use num_bigint::{BigInt, ToBigInt};

#[derive(Debug)]
pub struct Fibbonacci;

impl Fibbonacci {
    pub fn fibbo(num: BigInt) -> BigInt {
      let mut previous_number: BigInt = 1.to_bigint().unwrap();
      let mut current_number: BigInt = 0.to_bigint().unwrap();
      
      let mut  i = 0.to_bigint().unwrap();
      while i < num {
          let previous_previous_number = previous_number;
          previous_number = current_number + &previous_previous_number;
          current_number = previous_previous_number;
          i += 1;
      }
      current_number
    }
}