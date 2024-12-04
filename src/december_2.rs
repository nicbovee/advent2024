use std::{fs::File, io::{self, BufRead}};



fn is_safe(parts: &Vec<i32>) -> bool {
  let mut parts_sorted= parts.clone();
  parts_sorted.sort();

  let parts_descending: Vec<i32> = parts_sorted.iter().rev().cloned().collect();

  if parts != &parts_sorted && parts != &parts_descending {
    return false
  }

  for i in 1..parts.len(){
    let difference = parts[i] - parts[i-1];

    if difference.abs() == 0 || difference.abs() > 3{
      return false;
    } 
  }

  return true;
}

pub fn use_dampener(parts: &Vec<i32>)-> bool{
  if is_safe(parts) {
    return true;
  }

  for i in 0..parts.len(){
    let mut modified = parts.clone();
    modified.remove(i);

    if is_safe(&modified){
     return true 
    }
  }
  return false

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
  
        // Valid increasing sequence
        assert!(is_safe(&vec![1, 2, 3]));
        assert!(is_safe(&vec![1, 3, 4]));
        
        // Valid decreasing sequence
        assert!(is_safe(&vec![5, 4, 3]));
        assert!(is_safe(&vec![6, 4, 2]));

        
        assert!(!is_safe(&vec![6, 8, 2]));
        assert!(!is_safe(&vec![1, 8, 2, 3]));

        // No Duplicates
        assert!(!is_safe(&vec![1, 1, 2, 2, 3, 4]));
        
        // Invalid sequences
        assert!(!is_safe(&vec![1, 1, 2]));  // No zero difference allowed
        assert!(!is_safe(&vec![1, 5, 6]));  // Difference > 3
    }

    #[test]
    fn test_use_dampener() {
      // no duplicate duplicates
        assert!(!use_dampener(&vec![1, 1, 2, 2, 3, 4]));
        // one duplicate okay
        assert!(use_dampener(&vec![1,1, 2, 3, 4]));
        // one out of order item okay
        assert!(use_dampener(&vec![1, 2, 5, 3, 4]));
        // two out of order items not okay
        assert!(!use_dampener(&vec![1, 2, 5, 3, 4, 2]));
    }
}

pub fn part_1() {
  let file = File::open("src/december_2_input.txt").expect("should have read the file");
  let reader = io::BufReader::new(file);

  let answer = reader.lines()
  .flatten()
  .map(|line| line.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>())
  .filter(|parts| is_safe(parts)).count() as i32;

  println!("December 2 Answer:{}", answer);
}

// after getting stumpped on part 2 I leaned on this
// person's answer for help
// https://github.com/Fadi88/AoC/blob/master/2024/day02/main.rs
// I also created some tests to help me make more sense of this.
pub fn part_2(){

  let file = File::open("src/december_2_input.txt").expect("should have read the file");
  let reader = io::BufReader::new(file);

  let answer = reader.lines()
  .flatten()
  .map(|line| line.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>())
  .filter(|parts| use_dampener(parts)).count() as i32;

  println!("December 2 Answer:{}", answer);

}
