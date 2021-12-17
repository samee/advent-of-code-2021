use std::collections::HashMap;
use std::io::Read;

fn read_input() -> Vec<u8> {
  let mut rv = Vec::new();
  std::io::stdin().read_to_end(&mut rv).expect("No input");
  rv
}

fn endpoints(edge: &[u8]) -> (String, String) {
  let mut ep = edge.split(|&b| b == b'-');
  let v1 = String::from_utf8(ep.next().unwrap().to_vec()).unwrap();
  let v2 = String::from_utf8(ep.next().unwrap().to_vec()).unwrap();
  (v1, v2)
}

fn read_graph() -> HashMap<String, Vec<String>> {
  let mut rv = HashMap::new();
  let input = read_input();
  for edge in input.split(|&b| b == b'\n').filter(|&l| !l.is_empty()) {
    let (v1, v2) = endpoints(edge);
    let mut add_edge = |u1: &str, u2: String| {
      if !rv.contains_key(u1) { rv.insert(u1.to_string(), vec![u2]); }
      else { rv.get_mut(u1).unwrap().push(u2); }
    };
    add_edge(&v1, v2.clone());
    add_edge(&v2, v1);
  }
  rv
}

// path should have been Vec<&str>. Ran into some lifetime issue.
fn path_count(
  graph: &HashMap<String, Vec<String>>,
  start: &str,
  path: &mut Vec<String>) -> usize {
  if start == "end" { return 1; }
  if start.chars().next().unwrap().is_lowercase()
    && path.contains(&start.to_string()) { return 0; }
  path.push(start.to_string());
  let mut rv = 0;
  for neighbor in graph.get(start).unwrap_or(&Vec::new()) {
    rv += path_count(&graph, &neighbor, path);
  }
  path.pop();
  rv
}
fn main() {
  let graph = read_graph();
  let mut path = Vec::new();
  println!("Path count: {}", path_count(&graph, "start", &mut path));
}
