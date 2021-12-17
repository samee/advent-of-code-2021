use std::io::Read;

fn read_all() -> Vec<u8> {
  let mut inbytes = Vec::new();
  std::io::stdin().read_to_end(&mut inbytes).expect("Read failed");
  inbytes
}

fn height(map: &Vec<&[u8]>, r: isize, c: isize) -> usize {
  (map[r as usize][c as usize] - b'0') as usize
}

fn is_lower(h: usize, map: &Vec<&[u8]>, r: isize, c: isize) -> bool {
  if r < 0 || r >= map.len() as isize { true }
  else if c < 0 || c >= map[0].len() as isize { true }
  else { h < height(map, r, c) }
}

fn main() {
  let input = read_all();
  let map = input.split(|&b| b==b'\n').filter(|&l| !l.is_empty())
                 .collect::<Vec<_>>();
  let rows = map.len() as isize;
  let cols = map[0].len() as isize;
  let mut risk = 0;
  println!("Map size: {} rows and {} cols", rows, cols);
  for r in 0..rows {
    for c in 0..cols {
      let cur = height(&map, r, c);
      if is_lower(cur, &map, r-1, c) && is_lower(cur, &map, r+1, c)
        && is_lower(cur, &map, r, c-1) && is_lower(cur, &map, r, c+1) {
          risk += cur + 1
      }
    }
  }
  println!("Risk: {}", risk);
}
