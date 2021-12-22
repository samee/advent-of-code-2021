use std::io::{BufRead, stdin};
use std::ops::Range;

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
fn count_ons(reactor: &Vec<Vec<Vec<bool>>>,
             (xmap, ymap, zmap): &(Vec<i32>,Vec<i32>,Vec<i32>))
                                   -> u64 {
  let mut c = 0;
  for i in 0..reactor.len() {
    for j in 0..reactor[i].len() {
      for k in 0..reactor[j].len() {
        if reactor[i][j][k] {
          c += (xmap[i+1]-xmap[i]) as u64 *
               (ymap[j+1]-ymap[j]) as u64 *
               (zmap[k+1]-zmap[k]) as u64;
        }
      }
    }
  }
  c
}

fn map_range(r: (i32,i32), cmap: &[i32]) -> Range<usize> {
  let a = cmap.partition_point(|x| *x<r.0);
  let b = cmap.partition_point(|x| *x<r.1);
  a..b
}

fn exec(
  reactor: &mut Vec<Vec<Vec<bool>>>,
  cmaps: &(Vec<i32>, Vec<i32>, Vec<i32>),
  instr: Instr
  ) {
  for x in map_range(instr.xrange, &cmaps.0) {
    for y in map_range(instr.yrange, &cmaps.1) {
      for z in map_range(instr.zrange, &cmaps.2) {
        reactor[x][y][z] = instr.to_state_on;
      }
    }
  }
}

fn coord_maps(instr: &[Instr]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
  let mut xs = Vec::new();
  let mut ys = Vec::new();
  let mut zs = Vec::new();
  for ins in instr {
    xs.push(ins.xrange.0); xs.push(ins.xrange.1);
    ys.push(ins.yrange.0); ys.push(ins.yrange.1);
    zs.push(ins.zrange.0); zs.push(ins.zrange.1);
  }
  xs.sort();
  ys.sort();
  zs.sort();
  (xs, ys, zs)
}
fn main() {
  let instr = read_input();
  let cmaps = coord_maps(&instr);
  let mut reactor
    = vec![vec![vec![false; cmaps.0.len()-1]
                          ; cmaps.1.len()-1]
                          ; cmaps.2.len()-1];
  for ins in instr {
    exec(&mut reactor, &cmaps, ins);
  }
  println!("Count ons: {}", count_ons(&reactor, &cmaps));
}
