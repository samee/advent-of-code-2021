use std::io::BufRead;
use std::io::stdin;

fn parse(l: &str) -> (i64, i64) {
  let v : Vec<&str> = l.split(" ").collect();
  let d : i64 = v[1].parse().unwrap();
  if v[0] == "forward" { (d, 0) }
  else if v[0] == "up" { (0, -d) }
  else if v[0] == "down" { (0, d) }
  else { panic!("Bad command {}", v[0]) }
}

fn main() {
  let mut curx = 0;
  let mut cury = 0;
  let mut aim = 0;
  for (dd, daim) in stdin().lock().lines().map(|l| parse(&l.unwrap())) {
    if dd != 0 { curx += dd; cury += dd*aim; }
    else { aim += daim; }
    if cury < 0 { println!("Surface!"); }
  }
  println!("x: {}, y: {}, product: {}", curx, cury, curx*cury);
}
