use std::io::Read;

fn read_input() -> String {
  let mut buf = Vec::new();
  std::io::stdin().read_to_end(&mut buf).expect("Input not found");
  std::str::from_utf8(&buf).unwrap().to_owned()
}
#[derive(Debug,Clone)]
enum Fold {
  X(u32),
  Y(u32),
}
struct Input {
  dots : Vec<(u32,u32)>,
  folds : Vec<Fold>,
}
fn parse_input(inbytes: &str) -> Input {
  let mut input = Input{
    dots: Vec::new(),
    folds: Vec::new(),
  };
  for line in inbytes.split(|c| c == '\n') {
    if line.is_empty() { continue; }
    else if let Some(i) = line.find(|c| c == ',') {
      let x : u32 = line[0..i].parse().unwrap();
      let y : u32 = line[i+1..].parse().unwrap();
      input.dots.push((x,y));
    }else if let Some(fold) = line.strip_prefix("fold along ") {
      let pos : u32 = fold[2..].parse().unwrap();
      input.folds.push(
        if fold.starts_with("x") { Fold::X(pos) }
        else if fold.starts_with("y") { Fold::Y(pos) }
        else { panic!("extra dimension {}", fold.as_bytes()[0]) }
      );
    }else { panic!("Bad input line '{}'", line); }
  }
  input
}

fn fold(dots: &Vec<(u32,u32)>, fold: Fold) -> Vec<(u32,u32)> {
  dots.iter().map(|&(x,y)| {
    match fold {
      Fold::X(fx) => if x <= fx { (x, y) } else { (2*fx-x, y) },
      Fold::Y(fy) => if y <= fy { (x, y) } else { (x, 2*fy-y) },
    }
  }).collect()
}

fn plot_dots(dots: &Vec<(u32,u32)>) -> Vec<String> {
  let xmax = dots.iter().fold(0, |max, &(x,_)| max.max(x)) as usize;
  let ymax = dots.iter().fold(0, |max, &(_,y)| max.max(y)) as usize;
  let mut rv = vec![vec![b' '; xmax+1]; ymax+1];
  for &(x,y) in dots {
    rv[y as usize][x as usize]=b'#';
  }
  rv.iter()
    .map(|line| std::str::from_utf8(line).unwrap().to_string())
    .collect()
}

fn main() {
  let input = parse_input(&read_input());
  let mut dot2 = input.folds.iter().fold(
    input.dots,
    |a, finstr| fold(&a, finstr.clone()));

  dot2.sort(); dot2.dedup();
  for line in plot_dots(&dot2) {
    println!("{}", line);
  }
}
