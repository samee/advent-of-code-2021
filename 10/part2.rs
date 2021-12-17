use std::io::Read;

fn read_all() -> Vec<u8> {
  let mut inbytes = Vec::new();
  std::io::stdin().read_to_end(&mut inbytes).expect("Read failed");
  inbytes
}

fn is_open(c: u8) -> bool {
  [b'(', b'[', b'{', b'<'].contains(&c)
}
fn matches(open: Option<&u8>, cur: u8) -> bool {
  let opens = b"([{<";
  let closes = b")]}>";
  match open {
    None => false,
    Some(&prev) => {
      for i in 0..4 {
        if prev==opens[i] { return cur == closes[i] }
      }
      panic!("Bad symbol in input")
    }
  }
}
fn score_opening(c: u8) -> u64 {
  let scores = [(b'(',1), (b'[',2), (b'{',3), (b'<',4)];
  for (c2, sc) in scores { if c == c2 { return sc; }}
  panic!("{} is not an opening char", c);
}
fn incomplete_score(line: &[u8]) -> u64{
  let mut pending = Vec::<u8>::new();
  for &c in line {
    if is_open(c) { pending.push(c); }
    else if matches(pending.last(), c) { pending.pop(); }
    else { return 0; }
  }
  pending.reverse();
  pending.iter().fold(0, |a, &c| a*5+score_opening(c))
}

fn main() {
  let input = read_all();
  let lines : Vec<_> = input.split(|&c| c==b'\n').filter(|&l| !l.is_empty())
                            .collect();
  let mut scores =
    lines.iter().map(|&l| incomplete_score(l)).filter(|&x| x!=0)
         .collect::<Vec<_>>();
  scores.sort();
  println!("Median score: {}", scores[scores.len()/2]);
}
