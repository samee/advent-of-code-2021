use std::io::BufRead;
use std::io::stdin;

// Learning: Basic colletion ops such as map() and collect()
fn sliding(v: Vec<u32>) -> Vec<u32> {
  (2..v.len()).map(|i| v[i-2] + v[i-1] + v[i]).collect()
}
fn inccount(v: Vec<u32>) -> usize {
  (1..v.len()).filter(|i| {v[i-1] < v[*i]}).count()
}

fn main() {
  let v = stdin().lock().lines()
      .map(|l| l.unwrap().parse::<u32>().unwrap()).collect(); 
  println!("Sliding window increase: {}", inccount(sliding(v)));
}
