use std::io::stdin;

type SegMap = [[u8; 7]; 10];

static REF_MATRIX: SegMap = [
  [1, 1, 1, 0, 1, 1, 1],
  [0, 0, 1, 0, 0, 1, 0],
  [1, 0, 1, 1, 1, 0, 1],
  [1, 0, 1, 1, 0, 1, 1],
  [0, 1, 1, 1, 0, 1, 0],
  [1, 1, 0, 1, 0, 1, 1],
  [1, 1, 0, 1, 1, 1, 1],
  [1, 0, 1, 0, 0, 1, 0],
  [1, 1, 1, 1, 1, 1, 1],
  [1, 1, 1, 1, 0, 1, 1],
];

fn seg_map_input(s: &str) -> Box<SegMap> {
  let mut rv = Box::new([[0;7];10]);
  let mut i = 0;
  for digit in s.split(' ') {
    for c in digit.trim().bytes() {
      rv[i][(c as usize) - 97] = 1;
    }
    i += 1;
  }
  rv
}

/*
Brute-force searches for a permutation of 0..out.len() such that
good_so_far(out, out.len, arg) returns true. Returns immediately if one is
found. As an optimization, if good_so_far(out, prefix_len, arg) ever returns
false for a prefix_len < out.len, that prefix isn't explored any further.
   */
fn find_permute<T>(out: &mut[u8], arg: &T,
                   good_so_far: &impl Fn(&[u8], usize, &T)->bool) -> bool {
  for i in 0..out.len() { out[i] = i as u8; }
  find_permute_recur(out, 0, arg, good_so_far)
}
fn find_permute_recur<T>(out: &mut[u8], out_fixed: usize, arg: &T,
                         good_so_far: &impl Fn(&[u8], usize, &T)->bool)
  -> bool {
  if !good_so_far(out, out_fixed, arg) { return false; }
  else if out_fixed == out.len() { return true; }

  for i in out_fixed .. out.len() {
    out.swap(out_fixed, i);
    if find_permute_recur(out, out_fixed+1, arg, good_so_far) { return true; }
  }
  // Restore permutation
  for i in out_fixed+1 .. out.len() { out.swap(i-1, i); }
  false
}

struct SegPermState<'a> {
  in_matrix: &'a[[u8;7];10],
  perm: &'a[u8],
  fixed: usize,
}
fn digit_perm_good_so_far(digit_perm: &[u8], digit_fixed: usize,
                          seg: &SegPermState) -> bool {
  for si in 0..seg.fixed {
    for di in 0..digit_fixed {
      let perm_in = seg.in_matrix[di][seg.perm[si] as usize];
      if perm_in != REF_MATRIX[digit_perm[di] as usize][si] { return false; }
    }
  }
  true
}

fn seg_perm_good_so_far(seg_perm: &[u8], seg_fixed: usize,
                        in_matrix: &[[u8;7];10]) -> bool {
  // Optimization. Deleting this doesn't change answer
  if seg_fixed <= 3 { return true; }
  let mut digit_perm = [0; 10];
  find_some_good_digit_perm(seg_perm, seg_fixed, in_matrix, &mut digit_perm)
}
fn find_some_good_digit_perm(
  seg_perm: &[u8], seg_fixed: usize,
  in_matrix: &[[u8;7];10], digit_perm: &mut[u8; 10]) -> bool {
  let state = SegPermState{ in_matrix, perm: &seg_perm, fixed: seg_fixed };
  find_permute(digit_perm, &state, &digit_perm_good_so_far)
}
fn find_digit_perm(in_matrix: &[[u8;7];10], digit_perm: &mut[u8;10]) -> bool {
  let mut seg_perm = [0; 7];
  if !find_permute(&mut seg_perm, in_matrix, &seg_perm_good_so_far) {
    false
  } else {
    find_some_good_digit_perm(&seg_perm, 7, in_matrix, digit_perm)
  }
}

// .bytes() vs .as_bytes()?
fn has_same_chars(s: &str, t: &str) -> bool {
  if s.trim() != s || t.trim() != t {
    return has_same_chars(s.trim(), t.trim());
  }
  if s.len() != t.len() { return false; }
  let mut c = [0i8; 7];
  for i in 0..s.len() { c[s.as_bytes()[i] as usize - 97] += 1; }
  for i in 0..t.len() { c[t.as_bytes()[i] as usize - 97] -= 1; }
  c.iter().all(|&i| i == 0)
}

fn process_input() -> Option<(Box<SegMap>, Vec<u8>)> {
  let mut s = String::new();
  stdin().read_line(&mut s).ok()?;
  let pipe_pieces = s.split(" | ").collect::<Vec<_>>();
  if pipe_pieces.len() != 2 { return None; }

  let in_matrix = seg_map_input(pipe_pieces[0]);

  let all_digits = pipe_pieces[0].split(" ").filter(|s| s.len()!=0)
                                 .collect::<Vec<_>>();
  let to_pos
    = |s| all_digits.iter().position(|&t| has_same_chars(s,t)).unwrap() as u8;
  let queries : Vec<u8> = pipe_pieces[1].split(" ").map(&to_pos).collect();
  Some((in_matrix, queries))
}

fn to_int(digits: impl Iterator<Item=u8>) -> u32 {
  digits.fold(0, |a,d| a*10 + (d as u32))
}

fn tests() {
  fn gsf(out: &[u8], fixed: usize, _: &()) -> bool {
    if fixed < 3 { return true; }
    for i in 0..fixed { if out[i] != (9-i) as u8 { return false; } }
    return true;
  }
  let mut perm = [0; 10];
  if !find_permute(&mut perm, &(), &gsf) { panic!("Bad permute test"); }
  else { println!("Found permute {:?}", perm); }
}

fn main() {
  if true {
    let mut dperm = [0; 10];
    let mut total = 0;
    while let Some((in_matrix, queries)) = process_input() {
      find_digit_perm(&in_matrix, &mut dperm);
      total += to_int(queries.iter().map(|&i| dperm[i as usize]));
    }
    println!("Total {}", total);
  } else {
    tests();
  }
}
