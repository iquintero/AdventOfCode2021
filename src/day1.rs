use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const FILE_NAME: &str = "src/inputs/day1.txt";

fn main() {
  let mut count = 0;
  let mut prev = -1;

  if let Ok(lines) = read_lines(FILE_NAME) {
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
  }
  println!("total:\n{}", count);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
