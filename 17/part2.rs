struct Target {
  stx: i32,
  sty: i32,
  enx: i32,
  eny: i32,
}

fn in_target(curx: i32, cury: i32, t: &Target) -> bool {
  t.stx <= curx && curx <= t.enx && t.sty <= cury && cury <= t.eny
}

fn simulation_hits(mut vx: i32, mut vy: i32, t: &Target) -> bool {
  let mut curx = 0;
  let mut cury = 0;
  let mut highest = 0;
  while curx <= t.enx && cury >= t.sty {
    curx += vx;
    cury += vy;
    highest = highest.max(cury);
    if in_target(curx, cury, t) { return true; }
    if vx > 0 { vx -= 1 }
    vy -= 1;
  }
  false
}
fn option_count(t: Target) -> i32 {
  let mut rv = 0;
  for vx in 0..=t.enx {
    for vy in t.sty..=200 {
      if simulation_hits(vx, vy, &t) {
        rv += 1;
      }
    }
  }
  rv
}
fn parse_num_prefix(s: &str) -> i32 {
  let nums = match s.find(|c:char| c!='-' && !c.is_ascii_digit()) {
    None => s,
    Some(i) => &s[..i],
  };
  nums.parse().unwrap()
}

fn parse_range(s: &str) -> (i32,i32) {
  let st = parse_num_prefix(s);
  let enpos = s.find("..").unwrap();
  let en = parse_num_prefix(&s[enpos+2..]);
  (st, en)
}

fn read_line() -> Target {
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
  let xpos = line.find("x=").unwrap();
  let ypos = line.find("y=").unwrap();
  let (stx, enx) = parse_range(&line[xpos+2..]);
  let (sty, eny) = parse_range(&line[ypos+2..]);
  Target{stx, sty, enx, eny}
}

fn main() {
  println!("Option count: {}", option_count(read_line()));
}
