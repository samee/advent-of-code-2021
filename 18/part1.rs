use std::io::BufRead;
use std::ops::{Deref,DerefMut};

// Learn: Had a lot of truble destructuring Box of pair without moving
// out of it. r.deref() is not the same thing as *r.
// Not sure why in split() I'm allowed to borrow both x and t
#[derive(Debug)]
enum Tree {
  Leaf(u32),
  Pair(Box<(Tree,Tree)>),
}

fn parse_tree(line: &[u8], pos: &mut usize) -> Tree {
  if line[*pos].is_ascii_digit() {
    *pos += 1;  // assume single-digit numbers
    Tree::Leaf((line[*pos-1] - b'0') as u32)
  } else if line[*pos] != b'[' { panic!("Expected '[' at {}", *pos) }
  else {
    *pos += 1;
    let left = parse_tree(line, pos);
    *pos += 1;
    let right = parse_tree(line, pos);
    *pos += 1;
    Tree::Pair(Box::new((left, right)))
  }
}
fn add(a: Tree, b: Tree) -> Tree { Tree::Pair(Box::new((a,b))) }

fn add_to_leftmost(t: &mut Tree, v: u32) {
  match t {
    Tree::Leaf(leaf) => *leaf += v,
    Tree::Pair(bx) => {
      let (left, _) = bx.deref_mut();
      add_to_leftmost(left, v);
    }
  }
}
fn add_to_rightmost(t: &mut Tree, v: u32) {
  match t {
    Tree::Leaf(leaf) => *leaf += v,
    Tree::Pair(bx) => {
      let (_, right) = bx.deref_mut();
      add_to_rightmost(right, v);
    }
  }
}

// None means no explosion has happened.
// Some(0,0) means exposion has happened and has been resolved.
fn explode(t: &mut Tree, depth: u32) -> Option<(u32,u32)> {
  match t {
    Tree::Leaf(_) => None,
    Tree::Pair(bx) => {
      if depth >= 4 {
        let (&l, &r) = {
          if let (Tree::Leaf(l), Tree::Leaf(r)) = (*bx).deref() { (l,r) }
          else { panic!("Got a big tree") }
        };
        *t = Tree::Leaf(0);
        Some((l, r))
      }else {
        let (left, right) = bx.deref_mut();
        if let Some((l,r)) = explode(left, depth+1) {
          if r>0 { add_to_leftmost(right, r) }
          Some((l,0))
        }else if let Some((l,r)) = explode(right, depth+1) {
          if l>0 { add_to_rightmost(left, l) }
          Some((0,r))
        }else { None }
      }
    }
  }
}

fn split(t: &mut Tree) -> bool {
  match t {
    Tree::Leaf(x) => {
      if *x >= 10 {
        *t = Tree::Pair(Box::new((Tree::Leaf(*x/2), Tree::Leaf((*x+1)/2))));
        true
      }else { false }
    },
    Tree::Pair(bx) => {
      let (left, right) = bx.deref_mut();
      split(left) || split(right)
    }
  }
}

fn add_and_reduce(t1: Tree, t2: Tree) -> Tree {
  let mut res = add(t1, t2);
  loop {
    if let None = explode(&mut res, 0) {
      if !split(&mut res) { return res }
    }
  }
}

fn magnitude(t: &Tree) -> u32 {
  match t {
    &Tree::Leaf(x) => x,
    Tree::Pair(bx) => {
      let (l,r) = (*bx).deref();
      3*magnitude(&l) + 2*magnitude(&r)
    }
  }
}

fn main() {
  /*
  for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
    if line.is_empty() { continue }
    let mut tree = parse_tree(line.as_bytes(), &mut 0);
    explode(&mut tree, 0);
    println!("{:?}", tree);
  }
  */
  let mut res = None;
  for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
    if line.is_empty() { continue }
    let tree = parse_tree(line.as_bytes(), &mut 0);
    if let Some(acc) = res {
      res = Some(add_and_reduce(acc, tree));
    }else { res = Some(tree) }
  }
  if let Some(t) = res {
    println!("Add and reduce: {:?}, mag: {}", t, magnitude(&t));
  }else { panic!("No result"); }
}
