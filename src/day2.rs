use std::fs::File;
use std::io;

pub fn day2(lines: io::Lines<io::BufReader<File>>) {
  println!("Hello, Day2!");
  for line in lines {
    if let Ok(line) = line {
      println!("{}", line);
    }
  }
}