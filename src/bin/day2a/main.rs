fn max_cubes(s: &str) -> usize {
  match s {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
    _ => unreachable!(),
  }
}

fn main() {
  let s: usize = include_str!("input")
    .lines()
    .enumerate()
    .filter_map(|(g, l)| {
      l.split_once(": ")
        .unwrap()
        .1
        .split("; ")
        .all(|cubes| {
          cubes.split(", ").all(|color| {
            let (n, color) = color.split_once(' ').unwrap();
            n.parse::<usize>().unwrap() <= max_cubes(color)
          })
        })
        .then_some(g + 1)
    })
    .sum();
  println!("{}", s);
}
