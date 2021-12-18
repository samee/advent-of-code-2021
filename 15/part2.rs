use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Read;

fn read_input() -> Vec<u8> {
  let mut bytes = Vec::new();
  std::io::stdin().read_to_end(&mut bytes).unwrap();
  return bytes;
}

fn board_cost(board: &[&[u8]], r: isize, c: isize) -> u32 {
  let rows = board.len() as u32;
  let cols = board[0].len() as u32;
  let r1 = r as u32/rows;
  let r2 = r as u32%rows;
  let c1 = c as u32/cols;
  let c2 = c as u32%cols;
  let cost1 = (board[r2 as usize][c2 as usize] - b'0') as u32;
  return (cost1 - 1 + r1 + c1)%9 + 1;
}

fn shortest_path(board: &[&[u8]]) -> u32 {
  let rows = 5*board.len() as isize;
  let cols = 5*board[0].len() as isize;
  let mut visited = vec![vec![false; cols as usize]; rows as usize];
  let mut q = BinaryHeap::new();
  q.push((Reverse(0u32), 0, 0));
  while !q.is_empty() {
    let (Reverse(dist),r,c) = match q.pop() {
      Some(p) => p,
      None => panic!("Should be empty"),
    };
    if visited[r as usize][c as usize] { continue; }
    visited[r as usize][c as usize] = true;
    if r == rows-1 && c == cols-1 { return dist; }
    for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
      let r2 = r+dr;
      let c2 = c+dc;
      if r2 < 0 || r2 >= rows || c2 < 0 || c2 >= cols { continue; }
      let dd = board_cost(board, r2, c2);
      q.push((Reverse(dist+dd), r2, c2));
    }
  }
  return u32::MAX;
}

fn main() {
  let input = read_input();
  let board : Vec<&[u8]>
    = input.split(|&c| c==b'\n').filter(|&l| !l.is_empty()).collect();
  println!("Shortest path: {}", shortest_path(&board));
}
