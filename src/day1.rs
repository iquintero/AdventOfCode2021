use std::fs::File;
use std::io;

pub fn day1(lines: io::Lines<io::BufReader<File>>) {
  let mut count = 0;
  let mut prev = -1;

  for line in lines {
    if let Ok(number) = line {
      let current: i32 = number.trim().parse()
        .expect("invalid number");
      if prev != -1 && current > prev {
        count = count + 1;
      }
      prev = current;
    } else {
      break;
    }
  }
  println!("total:\n{}", count);
}
