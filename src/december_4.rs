use std::{fs::File, io::{self, BufRead}};

pub fn find_xmas(input: &String) ->i32{
  let mut total = 0;

  let n = input.matches("XMAS").count();

  total += n;

  let reversed: String = input.chars().rev().collect(); 

  let y = reversed.matches("XMAS").count();
  total += y;

  return total as i32;
  
}

pub fn part_1(){
  let file_path = "src/december_4_input.txt"; 

  let file = File::open(file_path).expect("should have read the file");
  let reader = io::BufReader::new(file);

  let lines: Vec<String> = reader.lines().flatten().collect();
  let mut total = 0;

  let mut columns_horizontal: Vec<Vec<String>> = Vec::new();

  
  for line in &lines{
    
    total += find_xmas(&line);

    let columns: Vec<String> = line.split("").filter(|s| !s.is_empty()).map(String::from).collect();
    
    if columns_horizontal.is_empty() {
        columns_horizontal = vec![Vec::new(); columns.len()];
    }
    
    for (i, c) in columns.into_iter().enumerate() {
        columns_horizontal[i].push(c);
    }
  }

  let mut columns_diagonal: Vec<Vec<String>> = Vec::new();
  let height = lines.len();
  let width = lines[0].len();

  // Collect diagonals going down-right (\)
  for start_col in 0..width {
    let mut diagonal = Vec::new();
    let mut row = 0;
    let mut col = start_col;
    
    while row < height && col < width {
      diagonal.push(lines[row].chars().nth(col).unwrap().to_string());
      row += 1;
      col += 1;
    }
    columns_diagonal.push(diagonal);
  }

  // Collect remaining down-right diagonals starting from first column
  for start_row in 1..height {
    let mut diagonal = Vec::new();
    let mut row = start_row;
    let mut col = 0;
    
    while row < height && col < width {
      diagonal.push(lines[row].chars().nth(col).unwrap().to_string());
      row += 1;
      col += 1;
    }
    columns_diagonal.push(diagonal);
  }

  // Collect diagonals going down-left (/)
  for start_col in 0..width {
    let mut diagonal = Vec::new();
    let mut row = 0;
    let mut col = start_col;
    
    while row < height && col >= 0 {
      diagonal.push(lines[row].chars().nth(col).unwrap().to_string());
      row += 1;
      if col > 0 { col -= 1; } else { break; }
    }
    columns_diagonal.push(diagonal);
  }

  // Collect remaining down-left diagonals starting from last column
  for start_row in 1..height {
    let mut diagonal = Vec::new();
    let mut row = start_row;
    let mut col = width - 1;
    
    while row < height && col > 0 {
      diagonal.push(lines[row].chars().nth(col).unwrap().to_string());
      row += 1;
      if col > 0 { col -= 1; } else { break; }
    }
    columns_diagonal.push(diagonal);
  }

  // Process the diagonals similar to horizontal
  for diagonal in columns_diagonal { 
    total += find_xmas(&diagonal.join(""));
  }

  for column in columns_horizontal{
    total += find_xmas(&column.join(""));
  }

  println!("December 4 part 1 answer: {}", total);
}

pub fn part_2(){
  let file_path = "src/december_4_input.txt"; 
  let file = File::open(file_path).expect("should have read the file");
  let reader = io::BufReader::new(file);

  let lines: Vec<String> = reader.lines().flatten().collect();

  let grid: Vec<Vec<char>> = lines
    .iter()
    .map(|line| line.chars().collect())
    .collect();

  let height = grid.len();
  let width = grid[0].len();

  let mut total = 0;

  for row in 1..height-1 {
    for col in 1..width-1 {
      let window = [
          [grid[row-1][col-1], grid[row-1][col], grid[row-1][col+1]],
          [grid[row][col-1], grid[row][col], grid[row][col+1]],
          [grid[row+1][col-1], grid[row+1][col], grid[row+1][col+1]],
      ];

      let diagonal1: String = vec![window[0][0], window[1][1], window[2][2]].iter().collect();
      let diagonal1_reversed = diagonal1.chars().rev().collect::<String>();

      let diagonal2: String = vec![window[0][2], window[1][1], window[2][0]].iter().collect();
      let diagonal2_reversed = diagonal2.chars().rev().collect::<String>();

      if (diagonal1.contains("MAS") || diagonal1_reversed.contains("MAS")) && (diagonal2.contains("MAS") || diagonal2_reversed.contains("MAS")){
        total += 1;
      }
    }
  }
  println!("December 4 part 2 answer: {}", total);
}