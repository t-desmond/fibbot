mod fib;
mod get_numbers;
mod manage_pr;
use fib::Fibbonacci;
use get_numbers::GetNumbers;
use num_bigint::BigInt;
use manage_pr::PullRequest;
use  dotenv::dotenv;
use std::env::{self};
#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>>{
    let _ = dotenv().is_ok();
    let github_token = env::var("GITHUB_TOKEN").unwrap_or_else(|_| "".to_string());
    let github_repository = env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "t-desmond/fibbot".to_string());
    println!("{}", github_token);
    let github_repository=  github_repository.split("/").collect::<Vec<&str>>();
    let owner = github_repository[0];
    let repo = github_repository[1];

    let pr = PullRequest::get_pr(&owner, &repo).await?;
    let path = &pr.items.first().unwrap().patch.clone().unwrap();
    
    println!("{:#?}", path);
    println!("{:?}", get_numbers::GetNumbers::extract_numbers(path));

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());
    println!("Enable Fibonacci Calculation: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    if enable_fib.eq("true") {
        println!("fibbot enabled...");
        let numbers = GetNumbers::extract_numbers(&path);
        let numbers: Vec<BigInt> = numbers
            .into_iter()
            .filter(|x| x < &max_threshold.parse::<u32>().unwrap())
            .map(|x| Fibbonacci::fibbo(x.into()))
            .collect();
        println!("{:?}", numbers);

        let comment_body = format!("{:?}", numbers);
        manage_pr::PullRequest::post_comment_to_pr(github_token.as_str(), owner, repo, 1, comment_body.as_str()).await?;
    } else {
        println!("fibbot disabled...")
    }
    Ok(())
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
        assert_eq!(GetNumbers::extract_numbers(" "), Vec::<u32>::new());
    }
}
