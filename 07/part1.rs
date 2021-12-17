use std::num::ParseIntError;
use std::str::FromStr;


fn parse_vec<T>(s: &str) -> Vec<T>
  where T: FromStr<Err = ParseIntError> {
  s.split(",").map(|x| x.trim().parse::<T>().unwrap()).collect()
}

fn main() {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).expect("Input line bad");
  let mut pos = parse_vec::<i32>(&line);
  pos.sort();
  println!("Input size: {}", pos.len());
  let dest = pos[pos.len()/2];
  println!("Everyone move to {}", dest);
  let cost : i32 = pos.iter().map(|x| (x-dest).abs()).sum();
  println!("Total cost to move is {}", cost);
}
