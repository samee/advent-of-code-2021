use std::io::stdin;

fn process_input() -> Option<usize> {
  let mut line = String::new();
  stdin().read_line(&mut line).ok()?;
  let pipe_pieces = line.split(" | ").collect::<Vec<_>>();
  if pipe_pieces.len() != 2 { return None; }

  let easy_sizes = &[2, 4, 3, 7];  // sizes of 1,4,7,8
  let rv = pipe_pieces[1].split(" ")
    .filter(|s| easy_sizes.contains(&s.trim().len()))
    .count();
  Some(rv)
}

fn main() {
  let mut total : usize = 0;
  while let Some(c) = process_input() { total += c; }
  println!("Total: {:?}", total);
}
