// Since I'm new to Rust, I'll leave comments on some things I either forgot about or learned for the first time.

use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};// <-- this was really confusing to me. 
// I need `self` because I call io on line 10 (could also have imported it on it's own line as std::io) 
// I need `BufRead` because I call `lines()` on line 15. I guess this is because that method only can be called if 
// the BufRead trait is present. 

// It seems like traits are kinda like interfaces in other languages I know. 
// the BufRead trait has some method signatures that get implmented for BufReader.
// this feels inconvenient to me especially since it's not clear to me what methods a trait is
// going to give me. I had been working without the rust analyzer so once I added that
// it was easier to see what methods were available through traits and auto-import them.

pub fn part_1() {
  let file = File::open("src/december_1_input.txt").expect("should have read the file");
  let reader = io::BufReader::new(file);
  let mut left_numbers = Vec::new();
  let mut right_numbers = Vec::new();

  for line in reader.lines().flatten(){

    // I guess with collect you have to have a method before it telling it
    // how to split something into a Vec. I've used chars before to get letters
    // from a string.
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() == 2 {
      // parse returns a result which can be Ok or Err
      // using `if let Ok` here allows me to do something with the
      // data coming back if the response is ok.
      // if I wanted to do something for both, I could use a match statement. 
      if let Ok(left) = parts[0].parse::<i32>() {
        left_numbers.push(left);
      }

      if let Ok(right) = parts[1].parse::<i32>()
      {
        right_numbers.push(right);
      }
    }
    
  }

  left_numbers.sort();
  right_numbers.sort();

  let mut final_total = 0;

  // still trying to wrap my head around borrowing, but I guess iter is needed
  // because of it borrows the values.

  // my understanding is that not using iter() here would mean that left_numbers
  // moves into the loop.

  // after reading more it seems like I don't really need to access left_numbers again
  // this explictely moves it into the for loop and it will be deallocated from memory
  // once the for loop is done.
  for (i, number) in left_numbers.into_iter().enumerate(){
    final_total += (number - right_numbers[i]).abs(); // <-- cool method that removes the sign on negatives
  }
  
  println!("December 1 Answer: {}", final_total);
}
pub fn part_2(){
  let file = File::open("src/december_1_input.txt").expect("should have read the file");
  let reader = io::BufReader::new(file);
  let mut left_numbers = Vec::new();
  let mut right_numbers = Vec::new();

  for line in reader.lines().flatten(){
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() == 2 {
      if let Ok(left) = parts[0].parse::<i32>() {
        left_numbers.push(left);
      }

      if let Ok(right) = parts[1].parse::<i32>()
      {
        right_numbers.push(right);
      }
    }
  }


  left_numbers.sort();
  right_numbers.sort();

  let mut count_map: HashMap<i32, i32> = HashMap::new();
  let mut final_total = 0;

  for &right_num in &right_numbers {
    *count_map.entry(right_num).or_insert(0) += 1;
  }
  
  for &left_num in &left_numbers {
    let count = count_map.get(&left_num).unwrap_or(&0);
    final_total += &left_num * count;
  }

  println!("December 2 Answer:{}", final_total);
}