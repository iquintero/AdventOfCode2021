use std::fs::File;
use std::io;

pub fn day3(lines: io::Lines<io::BufReader<File>>) {
  let mut numbers: Vec<String> = Vec::new();
  let mut gamma = 0;
  let mut epsilon = 0;

  for line in lines {
    if let Ok(line) = line {
      numbers.push(line);
    }
  }

  let size = numbers[0].len();
  let count = numbers.len() as u32;
  let mut ones: Vec<u32> = vec![0; size as usize];
  
  for n in numbers {
    for i in 0..size {
      if n.as_bytes()[i] == b'1' {
        ones[i] += 1;
      }
    }
  }
  let half = count / 2;
  for i in 0..size {
    gamma <<= 1;
    epsilon <<= 1;
    if ones[i] >= half {
      gamma += 1;
    } else {
      epsilon += 1;
    }
  }
  let answer = gamma * epsilon;
  println!("gamma: {}", gamma);
  println!("answer: {}", answer);
}