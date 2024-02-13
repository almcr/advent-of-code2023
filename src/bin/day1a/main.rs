fn main() {
  let n: u32 = include_str!("input")
    .lines()
    .map(|l| {
      l.chars().find_map(|c| c.to_digit(10)).unwrap() * 10
        + l.chars().rev().find_map(|c| c.to_digit(10)).unwrap()
    })
    .sum();

  println!("{}", n);
}
