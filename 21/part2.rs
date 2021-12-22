use std::io::{self, stdin};

fn read_line() -> Result<String, io::Error> {
    let mut rv = String::new();
    stdin().read_line(&mut rv)?;
    Ok(rv)
}

fn read_starting_pos() -> Result<(usize,usize), io::Error> {
  let line1 = read_line()?;
  let line2 = read_line()?;
  let pos1 =
    if let Some(suff) = line1.strip_prefix("Player 1 starting position: ") {
      suff.trim().parse::<usize>().unwrap()
    }else { panic!("Player 1 pos parsing"); };
  let pos2 =
    if let Some(suff) = line2.strip_prefix("Player 2 starting position: ") {
      suff.trim().parse::<usize>().unwrap()
    }else { panic!("Player 2 pos parsing"); };
  Ok((pos1, pos2))
}
fn wrap(s: usize) -> usize { (s-1)%10+1 }

type Memo = [[[[[(u64,u64);2];21];21];10];10];
fn simulate(pos1: usize, pos2: usize,
            score1: usize, score2: usize,
            cur_player: usize, memo: &mut Memo) -> (u64,u64) {
  if score1 >= 21 { return (1,0) }
  if score2 >= 21 { return (0,1) }
  let ref mut cache =
    memo[pos1-1][pos2-1][score1][score2][cur_player-1];
  if *cache != (0, 0) { return *cache }
  let mut rv = (0,0);
  for roll1 in 1..=3 {
    for roll2 in 1..=3 {
      for roll3 in 1..=3 {
        let sum : usize = roll1+roll2+roll3;
        let wincounts = 
          if cur_player == 1 {
            let newpos = wrap(pos1+sum);
            let newscore = score1+newpos;
            simulate(newpos, pos2, newscore, score2, 2, memo)
          }else {
            let newpos = wrap(pos2+sum);
            let newscore = score2+newpos;
            simulate(pos1, newpos, score1, newscore, 1, memo)
          };
        rv.0 += wincounts.0;
        rv.1 += wincounts.1;
      }
    }
  }
  // TODO see if we can reuse cache somehow
  memo[pos1-1][pos2-1][score1][score2][cur_player-1] = rv;
  rv
}
fn main() {
  let (pos1, pos2) = read_starting_pos().unwrap();
  let mut memo = Memo::default();
  let (win1, win2) = simulate(pos1, pos2, 0, 0, 1, &mut memo);
  println!("Universe counts: {} {}", win1, win2);
  println!("Max winner: {}", win1.max(win2));
}
