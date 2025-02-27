use regex::Regex;

pub struct GetNumbers;

impl GetNumbers {
    pub fn extract_numbers(pr_description: &str) -> Vec<u32> {
        let re = Regex::new(r"\b\d+\b").unwrap();
        re.find_iter(pr_description)
            .filter_map(|digits| digits.as_str().parse::<u32>().ok())
            .collect()
    }
}
