use std::io::Read;

fn read_all() -> String {
  let mut inbytes = Vec::new();
  std::io::stdin().read_to_end(&mut inbytes).expect("Read failed");
  std::str::from_utf8(&inbytes).unwrap().to_string()
}
fn main() {
  let mut count = [0u64; 9];
  for c in read_all().split(',').map(|s| s.trim().parse::<usize>().unwrap()) {
    count[c] += 1;
  }
  for _ in 0..256 {
    let mut new_count = [0u64; 9];
    new_count[6] += count[0];
    new_count[8] += count[0];
    for i in 1..9 { new_count[i-1] += count[i]; }
    for i in 0..9 { count[i] = new_count[i]; }
  }
  println!("Fish count after 256 days: {}", count.iter().sum::<u64>());
}
