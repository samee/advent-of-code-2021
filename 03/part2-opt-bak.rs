use std::io;
use std::io::BufRead;
use std::str;

/* Learning:
    * &T can by copied into T using *v
    * You can also just pattern-match as &v: |&v| {...}
    * Conversions between &str, String, &[u8], Vec<u8>
      https://stackoverflow.com/questions/41034635/
    * converting from binary string: usize::from_str_radix(i, 2)
    * It'd be nice to use bit_set crate,
      but I don't know how to use that without cargo.
    * Convert from Vec<&u32> to Vec<u32>: iter().cloned().collect()
    * Making vector of 0..n. (0..n).collect()
    * I should figure out how to use cargo for advent of code
*/ 

fn count_col(lines: &Vec<Vec<u8>>, inset: &Vec<usize>, col: usize) -> usize {
  inset.iter().filter(|&&i| lines[i][col] == b'1').count()
}

fn filter_down(lines: &Vec<Vec<u8>>, mut inset: Vec<usize>,
                max: bool, col: usize) -> &Vec<u8> {
  if inset.len() == 0 { panic!("Reached empty set"); }
  else if inset.len() == 1 { return &lines[inset[0]]; }

  let mostly_ones = count_col(lines, &inset, col) * 2 >= inset.len();
  let keep = if mostly_ones == max { b'1' } else { b'0' };
  inset.retain(|&l| lines[l][col] == keep);
  filter_down(lines, inset, max, col+1)
}

fn filter_down_to_int(lines: &Vec<Vec<u8>>, max: bool) -> usize {
  let line = filter_down(lines, (0..lines.len()).collect(), max, 0);
  usize::from_str_radix(str::from_utf8(line).unwrap(), 2).unwrap()
}

fn main() {
  let line_to_vec = |l: &str| l.as_bytes().to_vec();
  let lines : Vec<Vec<u8>>
    = io::stdin().lock().lines().map(|l| line_to_vec(&l.unwrap())).collect();
  let mx = filter_down_to_int(&lines.to_vec(), true);
  let mn = filter_down_to_int(&lines.to_vec(), false);

  println!("Life support rating: {}", mx*mn);
}
