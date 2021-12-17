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

fn measure_basin(map: &Vec<&[u8]>, r: isize, c: isize) -> usize {
  let mut q = Vec::<(isize,isize)>::new();
  let mut visited = vec![vec![false; map[0].len()]; map.len()];
  let mut visited_count = 0;
  q.push((r,c));
  while !q.is_empty() {
    let (r,c) = q.pop().unwrap();
    if r < 0 || r >= map.len() as isize ||
       c < 0 || c >= map[0].len() as isize { continue; }
    else if height(map, r, c) == 9 { continue; }
    else if visited[r as usize][c as usize] { continue; }
    visited[r as usize][c as usize] = true;
    visited_count += 1;
    let h = height(map, r, c);
    for (dr,dc) in [(-1,0), (0,1), (1,0), (0,-1)] {
      if is_lower(h, map, r+dr, c+dc) { q.push((r+dr, c+dc)); }
    }
  }
  visited_count
}

fn main() {
  let input = read_all();
  let map = input.split(|&b| b==b'\n').filter(|&l| !l.is_empty())
                 .collect::<Vec<_>>();
  let rows = map.len() as isize;
  let cols = map[0].len() as isize;
  let mut low_points = Vec::new();
  println!("Map size: {} rows and {} cols", rows, cols);
  for r in 0..rows {
    for c in 0..cols {
      let cur = height(&map, r, c);
      if is_lower(cur, &map, r-1, c) && is_lower(cur, &map, r+1, c)
        && is_lower(cur, &map, r, c-1) && is_lower(cur, &map, r, c+1) {
          low_points.push((r,c));
      }
    }
  }
  let mut basin = low_points.iter().map(|(r,c)| measure_basin(&map,*r,*c))
                            .collect::<Vec<_>>();
  basin.sort(); basin.reverse();
  let res : usize = basin[0..3].iter().product();
  println!("Low-count: {}, worst 3: {}", low_points.len(), res);
}
