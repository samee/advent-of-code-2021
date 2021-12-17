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
fn score_closing(c: u8) -> usize {
  let scores = [(b')',3), (b']',57), (b'}',1197), (b'>',25137)];
  for (c2, sc) in scores { if c == c2 { return sc; }}
  panic!("{} is not a closing char", c);
}
fn corrupted_score(line: &[u8]) -> usize {
  let mut pending = Vec::<u8>::new();
  for &c in line {
    if is_open(c) { pending.push(c); }
    else if matches(pending.last(), c) { pending.pop(); }
    else { return score_closing(c); }
  }
  0
}

fn main() {
  let input = read_all();
  let lines : Vec<_> = input.split(|&c| c==b'\n').filter(|&l| !l.is_empty())
                            .collect();
  println!("Corrupted score: {}",
           lines.iter().map(|&l| corrupted_score(l)).sum::<usize>());
}
