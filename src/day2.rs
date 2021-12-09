use std::fs::File;
use std::io;

pub fn day2(lines: io::Lines<io::BufReader<File>>) {
  let mut x = 0;
  let mut y = 0;
  for line in lines {
    if let Ok(line) = line {
      let v: Vec<&str> = line.split(' ').collect();
      let command = v[0];
      let amount: i32 = v[1].trim().parse().expect("invalid number");

      match command {
        "down" =>  y += amount,
        "up" => y -= amount,
        "forward" => x += amount,
        _ => {}
      }     
    }
  }
  let answer = x * y;
  println!("answer: {}", answer);
}