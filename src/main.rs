mod fib;
mod get_numbers;
mod manage_pr;
use dotenv::dotenv;
use fib::Fibbonacci;
use get_numbers::GetNumbers;
use manage_pr::PullRequest;
use num_bigint::BigInt;
use std::env::{self};
#[tokio::main]    
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv().is_ok();
    // get environment variables
    let github_repository =
        env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "t-desmond/fibbot".to_string());
    let github_repository_vec = github_repository.split("/").collect::<Vec<&str>>();
    let owner = github_repository_vec[0];
    let repo = github_repository_vec[1];
    let pr_number = env::var("PR_NUMBER")
        .unwrap_or_else(|_| "1".to_string())
        .parse::<u64>()
        .expect("Invalid PR_NUMBER");

    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or_else(|_| "true".to_string());
    if enable_fib.eq("true") {
        println!("fibbot enabled...");

        let max_threshold = env::var("INPUT_MAX_THRESHOLD").unwrap_or_else(|_| "100".to_string());
        println!("Max Threshold: {}", max_threshold);   

        // get pr content
        let pr = PullRequest::get_pr(&owner, &repo, pr_number).await?;
        let path = &pr.items.first().unwrap().patch.clone().unwrap();

        println!("Found : {:?}", get_numbers::GetNumbers::extract_numbers(path));

        let extracted_numbers = GetNumbers::extract_numbers(&path);
        
        let fib_of_extracted_numbers: Vec<BigInt> = extracted_numbers
            .into_iter()
            .filter(|x| x < &max_threshold.parse::<u32>().unwrap())
            .map(|x| Fibbonacci::fibbo(x.into()))
            .collect();
        println!("fib of found numbers less than {} are: {:?}", max_threshold, fib_of_extracted_numbers);

        let comment_body = format!("{:?}", fib_of_extracted_numbers);
    
        manage_pr::PullRequest::post_comment_to_pr(
            github_repository.as_str(),
            comment_body.as_str(),
            pr_number,
        )
        .await?;
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
