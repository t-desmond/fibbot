pub struct GetNumbers;

impl GetNumbers {
    pub fn extract_numbers(pr_content: &str) -> Vec<u32> {
      let txt_vec: Vec<String> = pr_content.split_whitespace().map(String::from).collect();

      let mut result: Vec<u32> = Vec::new();
      for i in txt_vec{
          if i.parse::<u32>().is_ok(){
              result.push(i.parse().unwrap());
          }
      }
      result
    }
}
