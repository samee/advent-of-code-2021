use std::io::{self, stdin, BufRead};

// u8 values are all variables: 0..4
// i64 can be intermediates
#[derive(Debug)]
enum Instr {
  Input(u8),
  Oper{op: BinOp, dest: u8, src: SrcValue},
}
#[derive(Debug)]
enum BinOp { Add, Mul, Div, Mod, Eql }
#[derive(Debug)]
enum SrcValue {
  Var(u8),
  Immed(i64),
}

fn read_instr() -> io::Result<Vec<Instr>> {
  let mut rv = Vec::new();
  for line_res in stdin().lock().lines() {
    let line = line_res?;
    if line.is_empty() { continue }
    rv.push(parse_instr(&line));
  }
  Ok(rv)
}

// TODO add custom error codes, and eliminate panics and unwraps.
fn parse_instr(line: &str) -> Instr {
  let toks : Vec<_>
    = line.split(|c| c == ' ').filter(|&t| !t.is_empty()).collect();
  let a: u8 = toks[1].as_bytes()[0] - b'w';
  if toks[0] == "inp" { Instr::Input(a) }
  else {
    let b =
      if toks[2].as_bytes()[0].is_ascii_alphabetic() {
        SrcValue::Var(toks[2].as_bytes()[0] - b'w')
      }else {
        SrcValue::Immed(toks[2].parse().unwrap())
      };
    let bop = match toks[0] {
      "add" => BinOp::Add,
      "mul" => BinOp::Mul,
      "div" => BinOp::Div,
      "mod" => BinOp::Mod,
      "eql" => BinOp::Eql,
      _ => panic!("unknown binary operation {}", toks[0]),
    };
    Instr::Oper{op: bop, dest: a, src: b}
  }
}

fn input_count(instr: &[Instr]) -> usize {
  instr.iter()
       .filter(|i| match i { Instr::Input(_) => true, _ => false })
       .count()
}
fn eval(instr: &[Instr], inputs: &[i64]) -> [i64;4] {
  let mut next_input = 0;
  let mut registers = [0; 4];
  for ins in instr {
    match ins {
      Instr::Input(reg) => {
        registers[*reg as usize] = inputs[next_input];
        next_input += 1;
      },
      Instr::Oper{op, dest, src} => {
        let v1 = registers[*dest as usize];
        let v2 = source_value(src, &registers);
        let res = match op {
          BinOp::Add => v1+v2,
          BinOp::Mul => v1*v2,
          BinOp::Div => v1/v2,
          BinOp::Mod => v1%v2,
          BinOp::Eql => if v1==v2 { 1 } else { 0 },
        };
        registers[*dest as usize] = res;
      }
    }
  }
  registers
}
fn source_value(src: &SrcValue, regs: &[i64; 4]) -> i64 {
  match src {
    &SrcValue::Var(reg) => regs[reg as usize],
    &SrcValue::Immed(i) => i,
  }
}

fn stringify(inputs: &[i64; 14]) -> String {
  String::from_utf8(
    inputs.iter().map(|i| b'0' + *i as u8).collect()
    ).unwrap()
}
fn next_input(inputs: &mut [i64; 14]) -> bool {
  for i in (0..14).rev() {
    if inputs[i] == 1 { inputs[i] = 9 }
    else { inputs[i] -= 1; return true }
  }
  false
}

#[allow(unused)]
fn parse_from_string(s: &str) -> Vec<Instr> {
  let mut rv = Vec::new();
  for line in s.lines() {
    if line.is_empty() { continue }
    rv.push(parse_instr(&line));
  }
  rv
}

fn tests() {
  let test1 = "inp x\nmul x -1";
  let res1 = eval(&parse_from_string(&test1), &[42])[1];
  assert!(res1 == -42);
  let test2 = "inp z\ninp x\nmul z 3\neql z x";
  let res2a = eval(&parse_from_string(&test2), &[2,6])[3];
  assert!(res2a == 1);
  let res2b = eval(&parse_from_string(&test2), &[3,6])[3];
  assert!(res2b == 0);
  let test3 = "
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
  let res3 = eval(&parse_from_string(&test3), &[12]);
  assert!(res3 == [1,1,0,0]);
  let mut inputs = [1,1,1,2,3,1,1,5,1,1,1,1,1,1];
  next_input(&mut inputs);
  assert!(inputs == [1,1,1,2,3,1,1,4,9,9,9,9,9,9]);
  println!("Tests passed!");
}

// Hand-coded from today's puzzle input.
// Option 1: compute this all manually. Hypothesis: input can be 9 for all
// addition cases. We can brute-force the rest.
fn manual_eval(input: &[i64;14]) -> i64 {
  fn b2i(b: bool) -> i64 { b as i64 }
  let  z0 = 0;
  let  z1 =  z0/ 1 + (25*( z0/ 1) + input[ 0]+ 2) * b2i(input[ 0]!= z0%26+10);
  let  z2 =  z1/ 1 + (25*( z1/ 1) + input[ 1]+16) * b2i(input[ 1]!= z1%26+15);
  let  z3 =  z2/ 1 + (25*( z2/ 1) + input[ 2]+ 9) * b2i(input[ 2]!= z2%26+14);
  let  z4 =  z3/ 1 + (25*( z3/ 1) + input[ 3]+ 0) * b2i(input[ 3]!= z3%26+15);
  let  z5 =  z4/26 + (25*( z4/26) + input[ 4]+ 1) * b2i(input[ 4]!= z4%26- 8);
  let  z6 =  z5/ 1 + (25*( z5/ 1) + input[ 5]+12) * b2i(input[ 5]!= z5%26+10);
  let  z7 =  z6/26 + (25*( z6/26) + input[ 6]+ 6) * b2i(input[ 6]!= z6%26-16);
  let  z8 =  z7/26 + (25*( z7/26) + input[ 7]+ 6) * b2i(input[ 7]!= z7%26- 4);
  let  z9 =  z8/ 1 + (25*( z8/ 1) + input[ 8]+ 3) * b2i(input[ 8]!= z8%26+11);
  let z10 =  z9/26 + (25*( z9/26) + input[ 9]+ 5) * b2i(input[ 9]!= z9%26- 3);
  let z11 = z10/ 1 + (25*(z10/ 1) + input[10]+ 9) * b2i(input[10]!=z10%26+12);
  let z12 = z11/26 + (25*(z11/26) + input[11]+ 3) * b2i(input[11]!=z11%26- 7);
  let z13 = z12/26 + (25*(z12/26) + input[12]+ 2) * b2i(input[12]!=z12%26-15);
  let z14 = z13/26 + (25*(z13/26) + input[13]+ 3) * b2i(input[13]!=z13%26- 7);
  // Matches [z4-z5, z6-z7, z3-z8, z9-z10, z11-z12, z2-z13, z1-z14]
  println!("Intermediate values: {:?}",
           [z1,z2,z3,z4,z5,z6,z7,z8,z9,z10,z11,z12,z13,z14]);
  z14
  /*
     Smallest:
     6, 1, 1, 9, 1, 5, 1, 6, 1, 1, 1, 3, 2, 1
  */
}
fn main() {
  tests();
  let instr = read_instr().unwrap();
  let mut inputs = [9; 14];
  if input_count(&instr) < inputs.len() { panic!("Not enough inputs") }
  let manual_input = [9, 8, 4, 9, 1, 9, 5, 9, 9, 9, 7, 9, 9, 4];
  let manual_input_small = [6, 1, 1, 9, 1, 5, 1, 6, 1, 1, 1, 3, 2, 1];
  println!("Eval with manual input: {}, manual_eval {}",
           eval(&instr, &manual_input_small)[3],
           manual_eval(&manual_input_small));
  /*
  for i in (0..14).rev() {
    let mut curmin = i64::MAX;
    let mut argmin = 0;
    for j in (1..=9).rev() {
      inputs[i] = j;
      let outputs = eval(&instr, &inputs);
      if outputs[3] < curmin { argmin = j; curmin = outputs[3] }
    }
    if argmin == 0 { panic!("argmin remained 0"); }
    inputs[i] = argmin;
  }
  println!("Serial number output: {}", eval(&instr, &inputs)[3]);
  println!("Serial number {} is valid", stringify(&inputs));
  let mut loop_count: i64 = 0;
  loop {
    let manual_output = manual_eval(&inputs);
    if manual_output == 0 {
      println!("Serial number {} is valid", stringify(&inputs));
      break;
    }
    /*
    let outputs = eval(&instr, &inputs);
    if outputs[3] != manual_output {
      panic!("Bad manual formula: input {:?} should produce {}, got {}",
             inputs, outputs[3], manual_eval(&inputs));
    }
    */
    if !next_input(&mut inputs) { panic!("No valid serial numbers") }
    loop_count += 1;
    if loop_count % 100000000 == 0 {
      println!("Finished {} loops", loop_count);
    }
  }
  */
}
