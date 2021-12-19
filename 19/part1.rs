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

// On error, returns the best overlap amount
fn has_overlap(set0: &Readings, seti: &Readings, xform: &Xform)
               -> Result<(), HasOverlapFailed> {
  let mut common = 0;
  for pt in seti {
    if set0.contains(&xform.apply(pt)) {
      common += 1;
      if common >= 12 { return Ok(()) }
    }
  }
  assert!(common>0);
  return Err(HasOverlapFailed(common))
}
fn all_rotations() -> [[i32;3];24] {
  fn neg_perm(mut l: [i32;3]) -> bool {
    let mut c = 0;
    while l[0].abs() > l[1].abs() || l[1].abs() > l[2].abs() {
      if l[0].abs() > l[1].abs() { l.swap(0, 1); c+=1 }
      if l[1].abs() > l[2].abs() { l.swap(1, 2); c+=1 }
    }
    [c==1 || c==3, l[0] < 0, l[1] < 0].iter().filter(|&v| *v).count() % 2 == 1
  }
  let mut rv = [[0;3];24];
  let mut i = 0;
  for x in -3i32..=3 {
    if x == 0 { continue }
    for y in -3i32..=3 {
      if y == 0 || x.abs() == y.abs() { continue }
      let zabs = 6-x.abs()-y.abs();
      let z = if neg_perm([x, y, zabs]) { -zabs } else {zabs};
      rv[i] = [x, y, z];
      i+=1;
    }
  }
  assert!(i==24);
  rv
}
fn find_overlap(
  seta: &Readings, setb: &Readings,
  rots: &[[i32;3];24]) -> Result<Xform, HasOverlapFailed> {
  let mut best_olap = 0;
  for rot in rots {
    for ptb in setb.iter().map(|pt| rotate(pt, rot)) {
      for pta in seta {
        let xform = Xform{
          offset: [pta[0]-ptb[0], pta[1]-ptb[1], pta[2]-ptb[2]],
          rot: rot.clone(),
        };
        match has_overlap(seta, setb, &xform) {
          Ok(()) => return Ok(xform),
          Err(HasOverlapFailed(olap)) => best_olap = best_olap.max(olap),
        }
      }
    }
  }
  Err(HasOverlapFailed(best_olap))
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
            let (r1,r2) = readings.split_at_mut(j);
            combine(&mut r1[i], &r2[0], &xform);
            r2[0].clear();
            println!("Combined {} <- {}, sizes: {} <- {},{}, overlap {}", i, j,
                     r1[i].len(), orig_sizes.0, orig_sizes.1,
                     orig_sizes.0+orig_sizes.1-r1[i].len());
            progress = true;
          },
          Err(HasOverlapFailed(x)) =>
            println!("  Best overlap between {} and {}: {}", i, j, x),
        }
      }
    }
    if !progress { break }
  }
  println!("Beacon count: {:?}",
           readings.iter().map(|r| r.len()).collect::<Vec<_>>());
}
