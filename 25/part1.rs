use std::io::{self, BufRead, stdin};

// TODO: Learn the iterator magic that allowed Result<> values to be collected()
fn read_board() -> io::Result<Vec<Vec<u8>>> {
  let mut rv = Vec::new();
  for lineres in stdin().lock().lines() {
    rv.push(lineres?.into_bytes());
  }
  Ok(rv)
}
// Try a more generic type
fn move_east(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let rows = board.len();
  let cols = board[0].len();
  let mut rv = vec![vec![b'.'; cols]; rows];
  for r in 0..rows {
    for c in 0..cols {
      let cf = if c==0 { cols-1 } else { c-1 };
      let ct = if c+1==cols { 0 } else { c+1 };
      if board[r][c] == b'.' && board[r][cf] == b'>' {
        rv[r][c] = b'>';
      }else if board[r][c] == b'>' && board[r][ct] == b'.' {
        rv[r][c] = b'.';
      }else { rv[r][c] = board[r][c]; }
    }
  }
  rv
}

fn move_south(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
  let rows = board.len();
  let cols = board[0].len();
  let mut rv = vec![vec![b'.'; cols]; rows];
  for r in 0..rows {
    for c in 0..cols {
      let rf = if r==0 { rows-1 } else { r-1 };
      let rt = if r+1==rows { 0 } else { r+1 };
      if board[r][c] == b'.' && board[rf][c] == b'v' {
        rv[r][c] = b'v';
      }else if board[r][c] == b'v' && board[rt][c] == b'.' {
        rv[r][c] = b'.';
      }else { rv[r][c] = board[r][c]; }
    }
  }
  rv
}

fn main() {
  let mut board = read_board().unwrap();
  let mut step_count = 0;
  loop {
    let board2 = move_south(&move_east(&board));
    step_count += 1;
    if board == board2 { break; }
    board = board2;
  }
  println!("Stability at step {}", step_count);
}
