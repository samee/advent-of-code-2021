use std::collections::HashMap;
use std::io::*;

type CountMap = HashMap<(u8,u8), u64>;

fn read_line() -> String {
  let mut rv = String::new();
  stdin().read_line(&mut rv).unwrap();
  rv.pop();  // The trailing newline
  return rv;
}

// AB -> C becomes b"ABC"
fn read_instr() -> Vec<[u8;3]> {
  let mut rv = Vec::new();
  for line in stdin().lock().lines().map(|l| l.unwrap()) {
    let b = line.as_bytes().to_owned();
    if b.is_empty() { continue; }
    if &b[2..6] != b" -> " { panic!("Invalid input line '{}'", line); }
    rv.push([b[0], b[1], b[6]]);
  }
  return rv;
}

fn in_instr(c1: u8, c2: u8, instr: &[[u8;3]]) -> Option<u8> {
  for &[d1,d2,d3] in instr {
    if c1 == d1 && c2 == d2 { return Some(d3); }
  }
  None
}

fn step(input: &CountMap, instr: &[[u8;3]])
  -> CountMap {
  let mut rv = HashMap::new();
  for (&(c1, c2), &count) in input.iter() {
    if let Some(c3) = in_instr(c1, c2, instr) {
      *rv.entry((c1, c3)).or_default() += count;
      *rv.entry((c3, c2)).or_default() += count;
    }else {
      *rv.entry((c1, c2)).or_default() += count;
    }
  }
  return rv;
}

fn show_stats(input: &CountMap, orig_input: &str) {
  let mut byte_counts = [0; 256];
  for (&(c1,c2), &count) in input {
    byte_counts[c1 as usize] += count;
    byte_counts[c2 as usize] += count;
  }
  let bytes = orig_input.as_bytes();
  byte_counts[bytes[0] as usize] += 1;
  byte_counts[bytes[bytes.len()-1] as usize] += 1;
  let (min,max) = byte_counts
    .iter()
    .fold((u64::MAX, u64::MIN), |(mn,mx), &c|
          if c == 0 { (mn, mx) }
          else { (mn.min(c/2), mx.max(c/2)) });
  println!("min: {}, max: {}, diff: {}", min, max, max-min);
}

fn count_pairs(s: &str) -> CountMap {
  let mut rv = HashMap::new();
  for l in s.as_bytes().windows(2) {
    *rv.entry((l[0], l[1])).or_default() += 1;
  }
  return rv;
}

fn main() {
  let input = read_line();
  let mut pairs = count_pairs(&input);
  read_line();
  let instr = read_instr();
  for _ in 0..40 {
    pairs = step(&pairs, &instr);
  }
  show_stats(&pairs, &input);
}
