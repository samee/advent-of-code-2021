use std::io::{self, stdin};

fn read_line() -> Result<String, io::Error> {
    let mut rv = String::new();
    stdin().read_line(&mut rv)?;
    Ok(rv)
}

fn read_starting_pos() -> Result<(u32,u32), io::Error> {
  let line1 = read_line()?;
  let line2 = read_line()?;
  let pos1 =
    if let Some(suff) = line1.strip_prefix("Player 1 starting position: ") {
      suff.trim().parse::<u32>().unwrap()
    }else { panic!("Player 1 pos parsing"); };
  let pos2 =
    if let Some(suff) = line2.strip_prefix("Player 2 starting position: ") {
      suff.trim().parse::<u32>().unwrap()
    }else { panic!("Player 2 pos parsing"); };
  Ok((pos1, pos2))
}
fn roll_die(s: &mut u32) -> u32 {
  *s += 1;
  (*s-1)%100+1
}
fn wrap(s: u32) -> u32 { (s-1)%10+1 }

fn main() {
  let (mut pos1, mut pos2) = read_starting_pos().unwrap();
  let mut score1 = 0;
  let mut score2 = 0;
  let mut die_state = 0;
  loop {
    let mut sum = roll_die(&mut die_state);
    sum += roll_die(&mut die_state);
    sum += roll_die(&mut die_state);
    pos1 = wrap(pos1+sum);
    score1 += pos1;
    if score1 >= 1000 { break }

    sum = roll_die(&mut die_state);
    sum += roll_die(&mut die_state);
    sum += roll_die(&mut die_state);
    pos2 = wrap(pos2+sum);
    score2 += pos2;
    if score2 >= 1000 { break }
  }
  println!("Scores: {} and {}, roll count: {}, output: {}",
           score1, score2, die_state, die_state*score1.min(score2));
}
