use std::io::*;

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

fn step(input: &str, instr: &[[u8;3]]) -> String {
  if input.len() == 1 { panic!("Single-char unimplemented"); }
  let mut rv = String::new();
  for w in input.as_bytes().windows(2) {
    match w {
      &[c1,c2] => {
        rv.push(c1 as char);
        if let Some(c3) = in_instr(c1,c2,instr) { rv.push(c3 as char); }
      },
      _ => { panic!("Bad window"); }
    }
  }
  rv.push(input.chars().last().unwrap());
  return rv;
}

fn show_stats(input: &str) {
  let mut counts = [0usize; 256];
  for &c in input.as_bytes() {
    counts[c as usize] += 1;
  }
  let (min,max) = counts.iter().fold((usize::MAX, usize::MIN), |(mn,mx), &c|
                                     if c == 0 { (mn, mx) }
                                     else { (mn.min(c), mx.max(c)) });
  println!("min: {}, max: {}, diff: {}", min, max, max-min);
}

fn main() {
  let mut s = read_line();
  read_line();
  let instr = read_instr();
  for _ in 0..10 {
    s = step(&s, &instr);
  }
  show_stats(&s);
}
