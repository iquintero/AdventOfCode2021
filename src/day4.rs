use std::collections::HashMap;
use std::fs::File;
use std::io;

fn parse_board(lines: io::Lines<io::BufReader<File>>) -> (Vec<u32>, Vec<HashMap<u32, (u32,u32)>>, Vec<Vec<i32>>) {
  let mut reading_bingo = true;
  let mut i = 0;
  let mut board_id = -1;
  let mut bingo: Vec<u32> = Vec::new();
  let mut board_keys: Vec<HashMap<u32, (u32, u32)>> = Vec::new();
  let mut boards: Vec<Vec<i32>> = Vec::new();
  for line in lines {
    if let Ok(line) = line {
      if reading_bingo {
        bingo = line.split(',').map(|x| x.trim().parse().expect("invalid number")).collect();
        reading_bingo = false;
        continue;
      }
      let items: Vec<&str> = line.split(' ').collect();
      if items.len() < 2 {
        i = 0;
        let keys: HashMap<u32, (u32, u32)> = HashMap::new();
        let board: Vec<i32> = vec![-1; 10];
        board_keys.push(keys);
        boards.push(board);
        board_id += 1;
        continue;
      }
      let mut j = 0;
      for n_str in items {
        if n_str.trim().len() == 0 {
          continue;
        }
        let n: u32 = n_str.trim().parse()
          .expect("invalid number");
        board_keys[board_id as usize].insert(n, (i, j));
        
        boards[board_id as usize][2 * i as usize] += 
          if n == 0 { 1i32 } else { n as i32 };
        boards[board_id as usize][(2 * j + 1) as usize] += 
          if n == 0 { 1i32 } else { n as i32 };
        j += 1;
      }
      i += 1;
    }
  }
  return (bingo, board_keys, boards);
}

pub fn day4(lines: io::Lines<io::BufReader<File>>) {
  let (bingo, board_keys, mut boards) = parse_board(lines);
  for n in bingo {
    println!("N ------ {} ------", n);
    for board_id in 0..boards.len() {
      if board_keys[board_id].contains_key(&n) {
        let (i, j) = board_keys[board_id].get(&n).unwrap();
        println!("{} {:?}", board_id, boards[board_id]);
        let col: usize = (2 * i) as usize;
        let row: usize = (2 * j + 1) as usize;
        boards[board_id][col] -= if n == 0 { 1i32 } else { n as i32 };
        boards[board_id][row] -= if n == 0 { 1i32} else { n as i32};
        if boards[board_id][col] == -1
         || boards[board_id][row] == -1 {
          let mut sum = 0;
          for id in 0..5 {
            println!("Adding: {}", boards[board_id][id]  + 1);
            sum += boards[board_id][id * 2] + 1;
          }
          println!("Sum: {}", sum);
          println!("N: {}", n);
          println!("Answer: {}", sum * n as i32);
          return;
        }
      }
    }
  }
}

pub fn day4_2(lines: io::Lines<io::BufReader<File>>) {
  let (bingo, board_keys, mut boards) = parse_board(lines);
  let mut active_boards = vec![true; boards.len()];
  let mut winner_board = 0;
  let mut winner_number = 0;
  for n in bingo {
    println!("N ------ {} ------", n);
    for board_id in 0..boards.len() {
      if active_boards[board_id] && board_keys[board_id].contains_key(&n) {
        let (i, j) = board_keys[board_id].get(&n).unwrap();
        println!("{} {:?}", board_id, boards[board_id]);
        let col: usize = (2 * i) as usize;
        let row: usize = (2 * j + 1) as usize;
        boards[board_id][col] -= if n == 0 { 1i32 } else { n as i32 };
        boards[board_id][row] -= if n == 0 { 1i32} else { n as i32};
        if boards[board_id][col] == -1
         || boards[board_id][row] == -1 {
           winner_board = board_id;
           winner_number = n;
           active_boards[board_id] = false;
         }
      }
    }
  }

  let mut sum = 0;
  for id in 0..5 {
    println!("Adding: {}", boards[winner_board][id]  + 1);
    sum += boards[winner_board][id * 2] + 1;
  }
  println!("Sum: {}", sum);
  println!("N: {}", winner_number);
  println!("Answer: {}", sum * winner_number as i32);
}

