use std::io::BufRead;
use std::io::stdin;

// Learning: how to read stdin lines till eof
//           how to parse to int and unwrap() errors
fn main() {
  let mut prev : u32 = u32::MAX;
  let mut larger : u32 = 0;
  for line in stdin().lock().lines() {
    let cur : u32 = line.unwrap().parse().unwrap();
    if prev < cur { larger += 1; }
    prev = cur;
  }
  println!("Larger count: {}", larger);
}
