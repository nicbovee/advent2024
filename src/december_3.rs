use regex::Regex;
use std::fs;


fn do_check(input: &str) -> i32{

  let do_these: Vec<&str> = input.split("do()").collect();
  let mut total = 0;

  for part in do_these {
    let do_not: Vec<&str> = part.split("don't()").collect();
    total += parse_multiply(do_not[0]);

  }
  return total; 

}

fn parse_multiply(input: &str) -> i32 {

  // this regex was more confusing than most regex in my book
  // because of all the parenth. 
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let mut total = 0;

  for caps in re.captures_iter(input){
      let arg1 = caps[1].to_string().parse::<i32>().unwrap_or(0);
      let arg2 = caps[2].to_string().parse::<i32>().unwrap_or(0);
      
      total += arg1 * arg2;

  }
  return total;
}

pub fn part_1(){
  let file_path = "src/december_3_input.txt"; 

    if let Ok(contents) = fs::read_to_string(file_path){
      let content_str: &str = &contents;  
      let total = parse_multiply(content_str);

      println!("December 3 part 1 answer: {}", total);
    };
}

pub fn part_2(){
  let file_path = "src/december_3_input.txt"; 
    if let Ok(contents) = fs::read_to_string(file_path){
      let content_str: &str = &contents;  
      let total = do_check(
        content_str
      );  
      println!("December 3 part 2 answer: {}", total);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiply() {
        // Valid increasing sequence
        assert!(parse_multiply("mul(2,2") == 0);
        assert!(parse_multiply("mul(2,2)") == 4);
        assert!(parse_multiply("mul(2,2)mul(2,8)") == 20);
    }
}


