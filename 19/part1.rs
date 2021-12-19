#![allow(dead_code)]

use std::collections::HashSet;
use std::io::BufRead;

type Readings = HashSet<[i32; 3]>;
struct Xform {
  offset: [i32;3],
  rot: [i32;3],
}
impl Xform {
  fn apply(self: &Self, pt: &[i32;3]) -> [i32;3] {
    let at = |i: i32| if i > 0 { pt[i as usize - 1] }
                      else { -pt[(-i) as usize -1] };
    let mut res = [0; 3];
    for i in 0..3 { res[i] = at(self.rot[i]) + self.offset[i] }
    return res
  }
}

fn at_mut<T>(arr: &mut[T], i1: usize, i2: usize) -> (&mut T, &mut T) {
  let (a1,a2) = arr.split_at_mut(i2);
  (&mut a1[i1], &mut a2[0])
}
fn rotate(pt: &[i32;3], rot: &[i32;3]) -> [i32;3] {
  Xform{offset: [0;3], rot: *rot}.apply(pt)
}
#[derive(Debug)]
struct HasOverlapFailed(i32);
impl std::error::Error for HasOverlapFailed {}
impl std::fmt::Display for HasOverlapFailed {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "HasOverlapFailed({})", self.0)
  }
}

fn count_overlap(set0: &Readings, seti: &Readings, xform: &Xform) -> i32 {
  let mut common = 0;
  for pt in seti {
    if set0.contains(&xform.apply(pt)) {
      common += 1;
    }
  }
  assert!(common>0);
  common
}
// This took way too much time and had way too many bugs
// The problem was to visually or easily confirm that there were no errors
fn all_rotations() -> [[i32;3];24] {
  fn one_count(i: usize) -> usize {
    if i==0 { 0 }
    else if i%2 == 0 { one_count(i/2) }
    else { 1+one_count(i/2) }
  }
  let perms = [
    [1,2,3], [1,3,2], [3,1,2],
    [3,2,1], [2,3,1], [2,1,3],
  ];
  let mut rv = [[0;3];24];
  let mut outc = 0;
  for i in 0..6 {
    for j in 0..8 {
      if (one_count(j) + i) % 2 == 1 { continue }
      rv[outc] = perms[i];
      for k in 0..3 {
        if j & (1<<k) != 0 { rv[outc][k] = -rv[outc][k] }
      }
      outc += 1;
    }
  }
  assert!(outc==24);
  rv
}
fn find_overlap(
  seta: &Readings, setb: &Readings,
  rots: &[[i32;3];24]) -> Result<Xform, HasOverlapFailed> {
  let mut best_olap = 0;
  let mut best_xform = Xform{offset: [0;3], rot: [1,2,3]};
  for rot in rots {
    //let mut firstb = true;
    for ptb in setb.iter().map(|pt| rotate(pt, rot)) {
      //if firstb { firstb = false; println!("    Rotated: {:?}", ptb); }
      for pta in seta {
        let xform = Xform{
          offset: [pta[0]-ptb[0], pta[1]-ptb[1], pta[2]-ptb[2]],
          rot: rot.clone(),
        };
        let olap = count_overlap(seta, setb, &xform);
        if olap > best_olap {
          best_olap = olap;
          best_xform = xform;
        }
      }
    }
  }
  if best_olap >= 12 { Ok(best_xform) }
  else { Err(HasOverlapFailed(best_olap)) }
}

fn read_scanner_readings() -> Vec<Readings> {
  let mut rv = Vec::<Readings>::new();
  let mut line_count = 0;
  for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
    if line.is_empty() { continue }
    if line.starts_with("---") {
      assert!(rv.last().map_or(0, |s| s.len()) == line_count);
      line_count = 0;
      rv.push(Readings::new());
    }
    else {
      let mut i = 0;
      let mut pt = [0i32; 3];
      for x in line.split(",").map(|s| s.trim().parse::<i32>().unwrap()) {
        pt[i] = x;
        i += 1;
      }
      rv.last_mut().unwrap().insert(pt);
      line_count += 1;
    }
  }
  rv
}
fn combine(seti: &mut Readings, setj: &Readings, xform: &Xform) {
  for pt in setj {
    seti.insert(xform.apply(pt));
  }
}

fn main() {
  let rots = all_rotations();
  let mut readings = read_scanner_readings();
  loop {
    let mut progress = false;
    for i in 0..readings.len() {
      if readings[i].is_empty() { continue }
      for j in i+1..readings.len() {
        if readings[j].is_empty() { continue }
        match find_overlap(&readings[i], &readings[j], &rots) {
          Ok(xform) => {
            let orig_sizes = (readings[i].len(), readings[j].len());
            let (ri,rj) = at_mut(&mut readings, i, j);
            combine(ri, rj, &xform);
            rj.clear();
            println!("Combined {} <- {}, sizes: {} <- {},{}, overlap {}", i, j,
                     ri.len(), orig_sizes.0, orig_sizes.1,
                     orig_sizes.0+orig_sizes.1-ri.len());

            progress = true;
          },
          Err(HasOverlapFailed(x)) if x > 1 =>
            println!("  Best overlap between {} and {}: {}", i, j, x),
          _ => (),
        }
      }
    }
    if !progress { break }
  }
  let beacon_count : Vec<_> = readings.iter().map(|r| r.len()).collect();
  assert!(beacon_count[1..].iter().all(|c| *c==0));
  println!("Beacon count: {}", beacon_count[0]);
}
