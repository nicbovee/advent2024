use std::{fs::File, io::{self, BufRead}};

pub fn part_1(){
  let file = File::open("src/december_2_input.txt").expect("should have read the file");
  let reader = io::BufReader::new(file);

  let mut answer = 0;
  for line in reader.lines().flatten(){
    let parts: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();

    let is_increasing = parts[0] - parts[1] < 0;

    let mut is_safe = true;
    for (i, number) in parts.iter().enumerate() {

      if i < parts.len() - 1 {
       let difference = (number - parts[i+1]);

        if (is_increasing && difference > 0) || (!is_increasing && difference < 0) {
          is_safe = false;
          break;
        }

        if difference.abs() > 3 || difference.abs() == 0{
          is_safe = false;
        }
      }
    }
    answer += if is_safe {1} else {0};


  }
  println!("December 2 Answer:{}", answer);
}

pub fn part_2(){

  let file = File::open("src/december_2_simple_input.txt").expect("should have read the file");
  let reader = io::BufReader::new(file);

  let mut answer = 0;
  for line in reader.lines().flatten(){
    let parts: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();

    let is_increasing = parts[0] - parts[1] < 0;

    let mut is_safe = true;
    let mut used_dampener: bool = false;
    let mut has_skipped = false;

    for (i, number) in parts.iter().enumerate() {

      if(used_dampener && !has_skipped){
        has_skipped = true;
        
        // println!("skipping: {}", number);
        continue;
      }
      if i < parts.len() - 1 {
       let difference = (number - parts[i+1]);


        if difference.abs() > 3 || difference.abs() == 0 || (is_increasing && difference > 0) || (!is_increasing && difference < 0) {

          println!("{:?}",parts);

          if i == parts.len() - 1 && !used_dampener && is_safe {
            break;
          }

          if i < parts.len() - 2 && is_safe
          {
            let next_difference = number - parts[i+2];
            if used_dampener || next_difference.abs() > 3 || next_difference.abs() == 0 || (parts.len() > 3 &&((is_increasing && next_difference > 0) || (!is_increasing && next_difference < 0))) {
              is_safe = false;
              println!("used dampener: {}, greater than 3: {}, no difference: {}, is decreasing when should increase:{}, is increasing when should decrease: {}", used_dampener, next_difference.abs() > 3 ,  next_difference.abs() == 0, (is_increasing && next_difference > 0), (!is_increasing && next_difference < 0) );
            } else {
              used_dampener = true;
              // println!("used dampener on {:?}", parts);
            }
          } else {
          is_safe = false;
          }
        }
      }
    }



    if(is_safe){
      println!("{:?}",parts);
    }

    answer += if is_safe {1} else {0};


  }
  println!("December 2 Answer:{}", answer);
}


// I'm stumped on this at the moment 
// atempted numbers
// 451
// 523
// 482
// 793
// 707