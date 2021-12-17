use std::io::BufRead;
use std::io::stdin;

fn skip_line(errmsg: &str) {
  stdin().read_line(&mut String::new()).expect(errmsg);
}

fn samples() -> Vec<u32> {
  let mut linebuf = String::new();
  stdin().read_line(&mut linebuf).expect("No samples");
  linebuf.trim().split(',').map(|s| s.parse().unwrap()).collect()
}

// TODO: I would have preferred Vec<Box<[[u32; 5]; 5]>>
// I would have preferred a flattened array
fn boards() -> Vec<Vec<Vec<u32>>> {
  let mut boards = vec![Vec::new()];
  for lineres in stdin().lock().lines() {
    let line = lineres.unwrap();
    if line.is_empty() { boards.push(Vec::new()); continue; }
    let row : Vec<u32> = line.split(' ').filter(|v| !v.is_empty())
                             .map(|v| v.parse().unwrap()).collect();
    boards.last_mut().unwrap().push(row);
  }
  boards
}

fn mark_sample(board: &Vec<Vec<u32>>, marks: &mut Vec<Vec<bool>>, samp: u32) {
  for row in 0..5 {
    for col in 0..5 {
      if board[row][col] == samp { marks[row][col] = true; }
    }
  }
}

fn row_marked(marks: &Vec<Vec<bool>>, row: usize)
  -> bool {
  for col in 0..5 { if !marks[row][col] { return false; }}
  return true;
}

fn col_marked(marks: &Vec<Vec<bool>>, col: usize)
  -> bool {
  for row in 0..5 { if !marks[row][col] { return false; }}
  return true;
}

fn score_winner(board: &Vec<Vec<u32>>, marks: &Vec<Vec<bool>>) -> Option<u32> {
  let mut won = false;
  for row in 0..5 { if row_marked(marks, row) { won = true; } }
  for col in 0..5 { if col_marked(marks, col) { won = true; } }
  if !won { return None }
  let mut res = 0;
  for row in 0..5 {
    for col in 0..5 {
      if !marks[row][col] { res += board[row][col]; }
    }
  }
  Some(res)
}

fn main() {
  let samples = samples();
  skip_line("No blank line after samples");
  let boards : Vec<Vec<Vec<u32>>> = boards();
  let mut marks = vec![vec![vec![false; 5]; 5]; boards.len()];
  for s in samples {
    for bi in 0..boards.len() {
      mark_sample(&boards[bi], &mut marks[bi], s);
      if let Some(score) = score_winner(&boards[bi], &marks[bi]) {
        println!("Score {}", score*s);
        return;
      }
    }
  }
}
