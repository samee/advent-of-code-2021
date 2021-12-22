use std::io::{BufRead, stdin};

#[derive(Debug)]
struct Instr {
  xrange: (i32,i32),
  yrange: (i32,i32),
  zrange: (i32,i32),
  to_state_on: bool,
}

fn parse_state(line: &mut String) -> &str {
  if line.starts_with("on ") {
    *line = line[3..].to_string();
    "on"
  }else if line.starts_with("off ") {
    *line = line[4..].to_string();
    "off"
  }else {
    panic!("Bad instruction: {}", line);
  }
}

fn parse_range(s: &str) -> (i32,i32) {
  let dots = s.find("..").unwrap();
  (s[2..dots].parse().unwrap(), s[dots+2..].parse::<i32>().unwrap()+1)
}

fn read_input() -> Vec<Instr> {
  let mut rv = Vec::new();
  for mut line in stdin().lock().lines().map(|l| l.unwrap()) {
    if line.is_empty() { continue }
    let to_state_on = {parse_state(&mut line) == "on"};
    let ranges: Vec<_> = line.split(",").map(&parse_range).collect();
    rv.push(Instr{
      xrange: ranges[0],
      yrange: ranges[1],
      zrange: ranges[2],
      to_state_on
    });
  }
  rv
}
fn count_ons(reactor: &[[[bool; 101]; 101]; 101]) -> u32 {
  let mut c = 0;
  for i in 0..101 {
    for j in 0..101 {
      for k in 0..101 {
        if reactor[i][j][k] { c += 1 }
      }
    }
  }
  c
}

fn clamp((mn,mx): (i32,i32)) -> (i32,i32) {
  (mn.clamp(-50,51), mx.clamp(-50,51))
}

fn exec(reactor: &mut[[[bool; 101];101];101], mut instr: Instr) {
  instr.xrange = clamp(instr.xrange);
  instr.yrange = clamp(instr.yrange);
  instr.zrange = clamp(instr.zrange);
  for x in instr.xrange.0..instr.xrange.1 {
    for y in instr.yrange.0..instr.yrange.1 {
      for z in instr.zrange.0..instr.zrange.1 {
        reactor[(x+50) as usize][(y+50) as usize][(z+50) as usize]
          = instr.to_state_on;
      }
    }
  }
}

fn main() {
  let instr = read_input();
  let mut reactor = [[[false; 101]; 101]; 101];
  for ins in instr {
    exec(&mut reactor, ins);
  }
  println!("Count ons: {}", count_ons(&reactor));
}
