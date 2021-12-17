use std::io::BufRead;
use std::io::stdin;

fn incr_vec(counts: &mut Vec<i32>, l: &[u8]) {
  counts.resize(l.len(), 0);
  for i in 0..l.len() {
    if l[i] == b'1' {counts[i] += 1;}
  }
}

fn negate(s: &str) -> String {
  s.chars().map(|c| if c=='0' { '1' } else {'0'}).collect()
}

fn main() {
  let mut v = Vec::new();
  let mut n = 0;
  for l in stdin().lock().lines() {
    incr_vec(&mut v, &l.unwrap().as_bytes());
    n += 1;
  }
  let g : String = v.iter().map(|c| if n<2*c { '0' } else { '1'} ).collect();
  let e = negate(&g);
  println!("Power draw: {}", isize::from_str_radix(&g,2).unwrap() *
                             isize::from_str_radix(&e,2).unwrap());
}
