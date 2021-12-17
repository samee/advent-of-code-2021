use std::io::Read;

fn read_input() -> std::io::Result<Vec<u8>> {
  let mut rv = Vec::new();
  std::io::stdin().read_to_end(&mut rv)?;
  Ok(rv)
}

fn incr_all(board: &mut [&mut [u8]]) {
  for r in 0..board.len() {
    for c in 0..board[0].len() {
      board[r][c] += 1;
    }
  }
}
fn in_bound(x:usize, dx:i8, xlim:usize) -> Option<usize> {
  let res = x as isize + dx as isize;
  if res < 0 || res >= xlim as isize  { None }
  else { Some(res as usize) }
}

fn flash_if_high(board: &mut [&mut [u8]]) -> usize {
  let rows = board.len();
  let cols = board[0].len();
  let mut flashed = vec![vec![false; cols]; rows];
  let mut flash_count = 0;
  let mut pending = Vec::<(usize,usize)>::new();
  for r in 0..rows {
    for c in 0..cols {
      if board[r][c] > 9 { pending.push((r,c)); }
    }
  }
  while let Some((r,c)) = pending.pop() {
    if flashed[r][c] { continue; }
    flashed[r][c] = true;
    flash_count += 1;
    for dr in -1..=1 {
      for dc in -1..=1 {
        if let ( Some(r2), Some(c2) ) =
          (in_bound(r, dr, rows), in_bound(c, dc, cols)) {
            board[r2][c2] += 1;
            if board[r2][c2] > 9 { pending.push((r2, c2)); }
          }
      }
    }
  }
  flash_count
}
fn reset_flashed(board: &mut [&mut [u8]]) {
  for row in board {
    for cell in row.iter_mut() {
      if *cell > 9 { *cell = 0 }
    }
  }
}

#[allow(dead_code)]
fn print_board(board: &[&mut [u8]]) {
  for row in board {
    for cell in row.iter() {
      print!("{:3}", *cell);
    }
    println!("");
  }
  println!("");
}

fn main() {
  let mut input = read_input().unwrap();
  let mut board = input.split_mut(|&c| c == b'\n')
                       .filter(|v| !v.is_empty())
                       .map(|v: &mut [u8]| {
                         for i in 0..v.len() { v[i] -= b'0'; }
                         v
                       })
                       .collect::<Vec<_>>();
  // This feels like too many write borrows. input and board
  // I don't understand any of these borrow rules. Why are we allowed to
  // have two mut refs? Why does board need to be mut, when we are only
  // modifying the pointee.

  let mut loop_count = 0;
  loop {
    incr_all(&mut board);
    // print_board(&board);
    let flash_count = flash_if_high(&mut board);
    // print_board(&board);
    reset_flashed(&mut board);
    // print_board(&board);
    loop_count += 1;
    if flash_count == 100 { break; }
  }
  println!("Sync count: {}", loop_count);
}
