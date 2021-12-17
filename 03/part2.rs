use std::io::BufRead;
use std::io::stdin;

fn count_col(lines: &Vec::<Vec<u8>>, col: usize) -> usize { 
  lines.iter().filter(|l| l[col] == b'1').count()
}

fn filter_down(lines: Vec::<Vec<u8>>, max: bool, col: usize) -> Vec<u8> {
  if lines.len() == 1 { return lines[0].to_vec(); }
  let keep = if (count_col(&lines, col) * 2 >= lines.len()) == max { b'1' }
             else { b'0' };
  let mut rv = Vec::<Vec<u8>>::new();
  for l in lines.iter() {
    if l[col] == keep { rv.push(l.to_vec()); }
  }
  return filter_down(rv, max, col+1);
}

fn filter_down_to_int(lines: &Vec<Vec<u8>>, max: bool) -> usize {
  usize::from_str_radix(std::str::from_utf8(
      &filter_down(lines.to_vec(), max, 0)).unwrap(), 2).unwrap()
}

fn main() {
  let mut lines = Vec::<Vec<u8>>::new();
  for l in stdin().lock().lines() {
    lines.push(l.unwrap().as_bytes().iter().map(|x| x+0).collect());
  }
  let mx = filter_down_to_int(&lines.to_vec(), true);
  let mn = filter_down_to_int(&lines.to_vec(), false);

  println!("Life support rating: {}", mx*mn);
}
