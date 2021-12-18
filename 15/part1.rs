use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Read;

fn read_input() -> Vec<u8> {
  let mut bytes = Vec::new();
  std::io::stdin().read_to_end(&mut bytes).unwrap();
  return bytes;
}

fn shortest_path(board: &[&[u8]]) -> u32 {
  let rows = board.len() as isize;
  let cols = board[0].len() as isize;
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
      let dd = (board[r2 as usize][c2 as usize]-b'0') as u32;
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
