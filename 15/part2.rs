use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::convert::TryInto;
use std::io::Read;

// Learnt about .wrapping_sub()
// Also had to bias dr,dc to prevent wraparound, explicitly relying on it.
fn read_input() -> Vec<u8> {
  let mut bytes = Vec::new();
  std::io::stdin().read_to_end(&mut bytes).unwrap();
  return bytes;
}

fn board_cost(board: &[&[u8]], r: usize, c: usize) -> u32 {
  let rows = board.len();
  let cols = board[0].len();
  let cost_shift : u32 = (r/rows + c/cols).try_into().unwrap();
  let cost1 : u32 = (board[r%rows][c%cols] - b'0').try_into().unwrap();
  return (cost1 - 1 + cost_shift)%9 + 1;
}

fn shortest_path(board: &[&[u8]]) -> u32 {
  let rows = 5*board.len();
  let cols = 5*board[0].len();
  let mut visited = vec![vec![false; cols]; rows];
  let mut q = BinaryHeap::new();
  q.push((Reverse(0u32), 0, 0));
  while !q.is_empty() {
    let (Reverse(dist),r,c) = match q.pop() {
      Some(p) => p,
      None => panic!("Should be empty"),
    };
    if visited[r][c] { continue; }
    visited[r][c] = true;
    if r+1 == rows && c+1 == cols { return dist; }
    for (dr, dc) in [(0,1), (1,2), (2,1), (1,0)] {
      let r2 = (r+dr).wrapping_sub(1);
      let c2 = (c+dc).wrapping_sub(1);
      if r2 >= rows || c2 >= cols { continue; }
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
