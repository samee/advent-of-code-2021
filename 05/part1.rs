use std::cmp::max;
use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;


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

// Learnt a new syntax. This means FromStr::Err = ParseIntError. I would have
// prefered braces, to be honest, since Err is a member. Or a separate
// T::Err = ParseIntError constraint altogether.
fn parse_vec<T>(s: &str) -> Vec<T>
  where T: FromStr<Err = ParseIntError> {
  s.split(",").map(|x| x.parse::<T>().unwrap()).collect()
}

/*
// Learning something new: you can collect() over iterator of Result<T,E> and
// have it produce Result<Vec<T>, E>. Using it is a bit hectic, though, since
// collect needs the full Result type annotation.
fn parse_vec_wrap<T>(s: &str) -> Result<Vec<T>, ParseIntError>
  where T: FromStr<Err = ParseIntError> {
  s.split(",").map(|x| x.parse::<T>()).collect()
}
*/

fn endpoints(line: &str) -> (i32, i32, i32, i32) {
  let points : Vec<Vec<i32>> = line.split(" -> ").map(parse_vec::<i32>)
                                   .collect();
  (points[0][0], points[0][1], points[1][0], points[1][1])
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
    if x1 == x2 || y1 == y2 {
      mark_line(&mut map, x1, y1, x2, y2);
    }
  }
  println!("Bad spot count: {}", bad_count(&map));
}
