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

pub fn day3_2(lines: io::Lines<io::BufReader<File>>) {
  let mut o2_n: Vec<String> = Vec::new();
  let mut co2_n: Vec<String> = Vec::new();

  for line in lines {
    if let Ok(line) = line {
      o2_n.push(line.clone());
      co2_n.push(line.clone());
    }
  }
  o2_n.sort();
  co2_n.sort();

  let size = o2_n[0].len();

  for i in 0..size {
    let count_o2 = o2_n.len() as u32;
    let half_o2 = count_o2 /2 ;
    let count_co2 = co2_n.len() as u32;
    let half_co2 = count_co2 / 2;
    let mut o2_id = 0;
    let mut co2_id = 0;
    for n in &o2_n {
      if n.as_bytes()[i] == b'1' {
        break;
      }
      o2_id += 1;
    }
  
    for n in &co2_n {
      if n.as_bytes()[i] == b'1' {
        break;
      }
      co2_id += 1;
    }

    if count_o2 > 1 {
      if o2_id  > half_o2 {
        let (new_o2_n, _) = o2_n.split_at(o2_id as usize);
        o2_n = new_o2_n.to_vec();
      } else {
        let (_, new_o2_n) = o2_n.split_at(o2_id as usize);
        o2_n = new_o2_n.to_vec();
      }
    }
    if count_co2 > 1 {
      if co2_id > half_co2 {
        let (_, new_co2_n) = co2_n.split_at(co2_id as usize);
        co2_n = new_co2_n.to_vec();
      } else {
        let (new_co2_n, _) = co2_n.split_at(co2_id as usize);
        co2_n = new_co2_n.to_vec();
      }
    }
    
  }
  let oxygen = isize::from_str_radix(&o2_n[0], 2).unwrap();
  let co2 = isize::from_str_radix(&co2_n[0], 2).unwrap();
  
  println!("Oxygen: {}", oxygen);
  println!("Co2: {}", co2);

  println!("answer: {}", oxygen * co2);

}