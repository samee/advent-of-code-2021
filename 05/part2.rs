use std::cmp::max;
use std::io::BufRead;


const MAXD: usize = 1000;

fn get_deltas(x1: i32, y1: i32, x2: i32, y2: i32)
  -> (i32, i32, i32) {
  let dx = if x1 > x2 { -1 } else if x1 == x2 { 0 } else { 1 };
  let dy = if y1 > y2 { -1 } else if y1 == y2 { 0 } else { 1 };
  let steps = max((x1-x2).abs(), (y1-y2).abs()) + 1;
  (dx, dy, steps)
}
fn mark_line(map: &mut[[i32; MAXD]],
             x1: i32, y1: i32, x2: i32, y2: i32) {
  let (dx, dy, steps) = get_deltas(x1, y1, x2, y2);
  for i in 0..steps {
    map[(x1+dx*i) as usize][(y1+dy*i) as usize] += 1;
  }
}

fn endpoints(line: &str) -> (i32, i32, i32, i32) {
  let point_str : Vec<&str> = line.split(" -> ").collect();
  let v1 : Vec<i32> = point_str[0].split(",")
                      .map(|x| x.parse::<i32>().unwrap()).collect();
  let v2 : Vec<i32> = point_str[1].split(",")
                      .map(|x| x.parse::<i32>().unwrap()).collect();
  (v1[0], v1[1], v2[0], v2[1])
}

fn bad_count(map: &[[i32; MAXD]]) -> i32 {
  let mut rv = 0;
  for r in 0..MAXD {
    for c in 0..MAXD {
      if map[r][c] >= 2 {
        rv += 1
      }
    }
  }
  rv
}

/*
fn debug_map(map: &[[i32; MAXD]]) {
  for r in 0..10 {
    for c in 0..10 {
      print!("{}", map[r][c]);
    }
    println!("");
  }
}
*/

fn main() {
  let mut map = [[0; MAXD]; MAXD];
  for lineres in std::io::stdin().lock().lines() {
    let line = lineres.unwrap();
    let (x1,y1,x2,y2) = endpoints(&line);
    mark_line(&mut map, x1, y1, x2, y2);
  }
  println!("Bad spot count: {}", bad_count(&map));
}
