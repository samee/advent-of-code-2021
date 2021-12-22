use std::io::{self, Read, stdin};

fn read_line() -> Result<Vec<u8>, io::Error> {
  let mut rv = String::new();
  stdin().read_line(&mut rv)?;
  Ok(rv.into_bytes())
}
fn read_to_end() -> Result<Vec<Vec<u8>>, io::Error> {
  let mut bytes = Vec::new();
  stdin().read_to_end(&mut bytes)?;
  let lines = bytes.split(|c| *c==b'\n')
                   .filter(|l| !l.is_empty())
                   .map(|l| l.to_owned())
                   .collect();
  Ok(lines)
}
// TODO generalize this &[Vec<u8>]
// TODO package this 'outside' byte a little better
fn at(board: &[Vec<u8>], r: isize, c: isize, outside: u8) -> u8 {
  if r < 0 || r as usize >= board.len() { outside }
  else if c < 0 || c as usize >= board[0].len() { outside }
  else { board[r as usize][c as usize] }
}
fn new_pixel(
  board: &[Vec<u8>],
  r: isize, c: isize,
  outside: u8, algoline: &[u8])
  -> u8 {
  let mut algoindex = 0;
  for i in r-1..=r+1 {
    for j in c-1..=c+1 {
      algoindex *= 2;
      if at(board, i, j, outside) == b'#' { algoindex += 1 }
    }
  }
  algoline[algoindex]
}
fn new_board(board: &[Vec<u8>], outside: u8, algoline: &[u8])
  -> (Vec<Vec<u8>>, u8) {
  let rows = board.len()+2;
  let cols = board[0].len()+2;
  let mut rv = vec![vec![b'.'; cols]; rows];
  for r in 0..rows {
    for c in 0..cols {
      rv[r][c] = new_pixel(board, r as isize - 1, c as isize - 1,
                           outside, algoline);
    }
  }
  (rv, algoline[0])
}

fn count_bright(board: &[Vec<u8>]) -> u32 {
  let mut count = 0;
  for r in 0..board.len() {
    for c in 0..board[r].len() {
      if board[r][c] == b'#' { count += 1 }
    }
  }
  count
}

#[allow(dead_code)]
fn show_board(board: &[Vec<u8>]) {
  for r in 0..board.len() {
    println!("{}", std::str::from_utf8(&board[r]).unwrap())
  }
}

fn main() {
  let algoline = read_line().unwrap();
  let mut board = read_to_end().unwrap();
  let mut outside = b'.';
  for _ in &[0, 1] {
    let (new_board, new_outside) = new_board(&board, outside, &algoline);
    board = new_board;
    outside = new_outside;
  }
  println!("Count bright: {}", count_bright(&board));
}
