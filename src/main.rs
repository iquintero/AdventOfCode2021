use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day1;
mod day2;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day = &args[1];
  let file = "src/inputs/".to_owned() + &day.to_owned() + ".txt";
 
  if let Ok(lines) = read_lines(file) {
    match day.as_ref() {
      "day1" => day1::day1(lines),
      "day2" => day2::day2(lines),
      _ => {}
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}