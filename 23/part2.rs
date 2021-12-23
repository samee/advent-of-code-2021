use std::convert::TryInto;
use std::cmp::Reverse;
use std::collections::{HashSet,BinaryHeap};

const HALLWAY_STOPS: &[u8;11] = b"  x x x x  ";
const ROOM_X: &[usize; 4] = &[2,4,6,8];

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Debug)]
struct Board {
  hallway: [u8; 11],
  side_rooms: [[u8;4]; 4],
}

fn absdiff(x: usize, y: usize) -> usize { if x > y { x-y } else { y-x } }
fn last_slot(side_room: &[u8;4]) -> usize {
  for i in 0..4 {
    if side_room[i] != b'.' { return i-1 }
  }
  3
}
fn absrange(hallway: &[u8], a: usize, b: usize) -> &[u8] {
  if a > b { &hallway[b..=a] } else { &hallway[a..=b] }
}

fn can_move_to_dest(board: &Board, hpos: usize) -> Option<u32> {
  if board.hallway[hpos] == b'.' { return None }
  let amphi_type = board.hallway[hpos];
  let dest_room = (amphi_type - b'A') as usize;
  if !board.side_rooms[dest_room].iter().all(|&c| c==amphi_type || c==b'.') {
    return None
  }
  if !absrange(&board.hallway, hpos, ROOM_X[dest_room]).iter()
      .all(|&c| c==amphi_type || c==b'.') {
    return None
  }
  let cost = absdiff(hpos, ROOM_X[dest_room]) + 1 +
    last_slot(&board.side_rooms[dest_room]);
  Some(cost_factor(amphi_type) * cost as u32)
}
fn move_to_dest(board: &Board, hpos: usize) -> Board {
  let mut new_board = board.clone();
  let amphi_type = board.hallway[hpos];
  let dest_room = (amphi_type - b'A') as usize;
  let slot = last_slot(&board.side_rooms[dest_room]);
  new_board.side_rooms[dest_room][slot] = amphi_type;
  new_board.hallway[hpos] = b'.';
  new_board
}
fn can_move_to_hallway(board: &Board, hpos: usize,
                       room_idx: usize, rslot: usize) -> Option<u32> {
  if !absrange(&board.hallway, hpos, ROOM_X[room_idx]).iter()
      .all(|&c| c==b'.') {
    return None
  }
  if HALLWAY_STOPS[hpos] != b' ' { return None }
  let cost = absdiff(hpos, ROOM_X[room_idx]) + 1 + rslot;
  Some(cost_factor(board.side_rooms[room_idx][rslot]) * cost as u32)
}
fn move_to_hallway(
  board: &Board, hpos: usize,
  room_idx: usize, rslot: usize) -> Board {
  let mut new_board = board.clone();
  new_board.hallway[hpos] = board.side_rooms[room_idx][rslot];
  new_board.side_rooms[room_idx][rslot] = b'.';
  new_board
}
fn all_sorted(board: &Board) -> bool {
  for i in 0..4 {
    for j in 0..4 {
      if board.side_rooms[i as usize][j] != b'A'+i { return false }
    }
  }
  true
}
const COST_FACTOR: [u32; 4] = [1, 10, 100, 1000];
fn cost_factor(at: u8) -> u32 { COST_FACTOR[(at-b'A') as usize] }

fn sorting_energy(board_init: Board) -> u32 {
  let mut q = BinaryHeap::new();
  let mut visited = HashSet::new();
  q.push((Reverse(0), board_init));
  while let Some((Reverse(cost), board)) = q.pop() {
    if !visited.insert(board.clone()) { continue }
    if all_sorted(&board) { return cost }
    // Hallway to room
    for i in 0..11 {
      if board.hallway[i] != b'.' {
        if let Some(delta_cost) = can_move_to_dest(&board, i) {
          q.push((Reverse(cost+delta_cost), move_to_dest(&board, i)))
        }
      }
    }
    // Room to hallway
    for j in 0..4 {
      for i in 0..11 {
        if HALLWAY_STOPS[i] != b' ' { continue }
        for k in 0..4 {
          if board.side_rooms[j][k] == b'.' { continue }
          if let Some(delta_cost) = can_move_to_hallway(&board, i, j, k) {
            q.push((Reverse(cost+delta_cost),
                    move_to_hallway(&board, i, j, k)));
          }
          break;
        }
      }
    }
  }
  panic!("No way to sort")
}

fn read_board() -> Result<Board, Box<dyn std::error::Error>> {
  let mut input = [String::new(), String::new(),
                   String::new(), String::new(), String::new()];
  for i in 0..5 {
    std::io::stdin().read_line(&mut input[i])?;
  }
  let mut board = Board{hallway: [0; 11], side_rooms: [[0;4];4]};
  board.hallway = input[1].as_bytes()[1..12].try_into()?;
  for i in 0..4 {
    board.side_rooms[i][0] = input[2].as_bytes()[1+ROOM_X[i]];
    board.side_rooms[i][1] = b" #D#C#B#A#"[ROOM_X[i]];
    board.side_rooms[i][2] = b" #D#B#A#C#"[ROOM_X[i]];
    board.side_rooms[i][3] = input[3].as_bytes()[1+ROOM_X[i]];
  }
  Ok(board)
}

fn print_board(board: &Board) {
  println!("#############");
  println!("#{}#", std::str::from_utf8(&board.hallway).unwrap());
  println!("###{}###",
           (0..4).map(|i| (board.side_rooms[i][0] as char).to_string())
                 .collect::<Vec<_>>()
                 .join("#"));
  for j in 1..4 {
    println!("  #{}#",
             (0..4).map(|i| (board.side_rooms[i][j] as char).to_string())
                   .collect::<Vec<_>>()
                   .join("#"));
  }
  println!("  #########");
}

fn main() {
  let board = read_board().unwrap();
  print_board(&board);
  println!("Sorting energy: {}", sorting_energy(board.clone()));
}
