use std::num::ParseIntError;
use std::str::FromStr;


fn parse_vec<T>(s: &str) -> Vec<T>
  where T: FromStr<Err = ParseIntError> {
  s.split(",").map(|x| x.trim().parse::<T>().unwrap()).collect()
}

fn cost_at(mypos: i32, pos: &[i32]) -> i32 {
  pos.iter().map(|x| {
    let d = (x-mypos).abs();
    d*(d+1)/2
  }).sum()
}

fn binary_search_range<F>(mut mn: i32, mut mx: i32, f: F) -> i32
  where F: Fn(i32) -> std::cmp::Ordering {
  while mn < mx {
    let cur = (mn+mx)/2;
    let cmp = f(cur);
    if cmp.is_lt() { mn = cur+1 }
    else if cmp.is_gt() { mx = cur }
    else { return cur }
  }
  mn
}

// Learnt:
// * iter() gets consumed by min() or max(), and that they return Option<&T>.
// * It's probably best to write your own binary search if you
//   want to do it over a Range object. Range object is _not_ binary searchable.
// * Maybe pattern-matching requires qualified identifiers for constructors.
fn min_dist(pos: &[i32]) -> i32 {
  let mn = *pos.first().unwrap();
  let mx = *pos.last().unwrap();
  let cost_balance = |q: i32| pos.iter().map(|p| q-p).sum::<i32>().cmp(&0);
  let cand = binary_search_range(mn, mx, cost_balance);
  // The binary search finds the first sign of increasing cost,
  // which may not be the minimum.
  if cost_at(cand, pos) > cost_at(cand-1, pos) { cand-1 }
  else { cand }
}

fn main() {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).expect("Input line bad");
  let mut pos = parse_vec::<i32>(&line);
  pos.sort();
  println!("Input size: {}", pos.len());
  let dest = min_dist(&pos);
  println!("Everyone move to {}", dest);
  println!("Total cost to move is {}", cost_at(dest, &pos));
}
