use std::convert::TryInto;
use std::io::BufRead;

fn to_bin(mut b: u8, out: &mut[u8; 4]) {
  for i in 0..4 {
    out[3-i] = b%2;
    b/=2;
  }
}
fn bitblast(s: &[u8]) -> Vec<u8> {
  let mut rv = Vec::new();
  let mut nibble = [0; 4];
  for b in s {
    if b.is_ascii_digit() { to_bin(b-b'0', &mut nibble); }
    else { to_bin(b-b'A'+10, &mut nibble); }
    rv.extend_from_slice(&nibble);
  }
  rv
}

// TODO find a better way to deserialize structs
#[derive(Debug)]
struct Packet {
  #[allow(dead_code)]
  version: u8,  // 3 bits
  packet_body: PacketBody,
}

#[derive(Debug)]
enum PacketBody {
  LiteralInt(u64),  // hopefully no bigint needed
  // Not sure if I have to distinguish between the two length types
  Op(PacketOp, Vec<Packet>),
}

#[derive(Debug)]
enum PacketOp {
  Sum, Product, Min, Max, Gt, Lt, Eq
}

fn to_int(bits: &[u8]) -> u64 {
  bits.iter().fold(0, |a,&b| a*2+b as u64)
}

fn parse_lit_int(bits: &[u8], pos: &mut usize) -> u64 {
  let mut rv = 0;
  for ck in bits[*pos..].chunks(5) {
    rv = 16*rv + to_int(&ck[1..]);
    *pos += 5;
    if ck[0] == 0 { return rv; }
  }
  panic!("Expected end chunk for int");
}

fn from_type_id(t: u64) -> PacketOp {
  match t {
    0 => PacketOp::Sum,
    1 => PacketOp::Product,
    2 => PacketOp::Min,
    3 => PacketOp::Max,
    5 => PacketOp::Gt,
    6 => PacketOp::Lt,
    7 => PacketOp::Eq,
    _ => panic!("Invalid type ID: {}", t)
  }
}

fn parse(bits: &[u8], pos: &mut usize) -> Packet {
  let version : u8 = to_int(&bits[*pos..][0..3]).try_into().unwrap();
  let typeid = to_int(&bits[*pos..][3..6]);
  *pos += 6;
  if typeid == 4 {
    Packet{
      version,
      packet_body: PacketBody::LiteralInt(parse_lit_int(bits, pos))
    }
  }else {
    let length_type = bits[*pos];
    let op = from_type_id(typeid);
    *pos += 1;
    if length_type == 0 {
      let sublen : usize = to_int(&bits[*pos..][..15]).try_into().unwrap();
      *pos += 15;
      let mut subpos = *pos;
      let mut subpacks = Vec::new();
      while subpos < *pos + sublen {
        subpacks.push(parse(bits, &mut subpos));
      }
      *pos += sublen;
      Packet{
        version,
        packet_body: PacketBody::Op(op, subpacks),
      }
    } else {
      let sublen : usize = to_int(&bits[*pos..][..11]).try_into().unwrap();
      *pos += 11;
      let mut subpacks = Vec::new();
      for _ in 0..sublen {
        subpacks.push(parse(bits, pos));
      }
      Packet{ version, packet_body: PacketBody::Op(op, subpacks) }
    }
  }
}

fn eval(pack: &Packet) -> u64 {
  match &pack.packet_body {
    PacketBody::LiteralInt(x) => *x,
    PacketBody::Op(op, subpacks) =>
      match &op {
        PacketOp::Sum => subpacks.iter().map(&eval).sum(),
        PacketOp::Product => subpacks.iter().map(&eval).product(),
        PacketOp::Min => subpacks.iter().map(&eval).min().unwrap(),
        PacketOp::Max => subpacks.iter().map(&eval).max().unwrap(),
        PacketOp::Gt => (eval(&subpacks[0]) > eval(&subpacks[1])) as u64,
        PacketOp::Lt => (eval(&subpacks[0]) < eval(&subpacks[1])) as u64,
        PacketOp::Eq => (eval(&subpacks[0]) == eval(&subpacks[1])) as u64,
      }
  }
}

fn main() {
  for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
    if line.is_empty() { continue }
    let input = parse(&bitblast(&line.as_bytes()), &mut 0);
    println!("Input value: {}", eval(&input));
  }
}
// TODO find dead code suppression
