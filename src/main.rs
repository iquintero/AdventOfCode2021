use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
  let args: Vec<String> = env::args().collect();
  let day = &args[1];
  let day_for_file = day.split('_').collect::<Vec<&str>>()[0];
  let file = "src/inputs/".to_owned() + &day_for_file.to_owned() + ".txt";
 
  if let Ok(lines) = read_lines(file) {
    match day.as_ref() {
      "day1" => day1::day1(lines),
      "day2" => day2::day2(lines),
      "day2_2" => day2::day2_2(lines),
      "day3" => day3::day3(lines),
      "day3_2" => day3::day3_2(lines),
      "day4" => day4::day4(lines),
      _ => {}
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}