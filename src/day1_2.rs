use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

// Part 2 input is the same as Day1
const FILE_NAME: &str = "src/inputs/day1.txt";

fn main() {
  let mut count = 0;
  let mut prev_sum = 0;
  let mut sum = 0;
  let mut started = false;

  if let Ok(lines) = read_lines(FILE_NAME) {
    let mut queue: VecDeque<u32> = VecDeque::new();

    for line in lines {
        if let Ok(number) = line {
          let current: u32 = number.trim().parse()
            .expect("invalid number");
          
          queue.push_back(current);
          sum = sum + current;
          if queue.len() > 3 && !started {
            started = true;
          }
          if started {
            sum -= queue.pop_front().unwrap();
            if sum > prev_sum {
              count += 1;
            }
          }
          prev_sum = sum;
        } 
    }
  }
  println!("total:\n{}", count);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
