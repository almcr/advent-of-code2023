const DIGIT_STR: &[&str] = &[
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
  let n: usize = include_str!("input")
    .lines()
    .map(|l| {
      (0..l.len()).find_map(|i| start_num(l, i)).unwrap() * 10
        + (0..l.len()).rev().find_map(|i| start_num(l, i)).unwrap()
    })
    .sum();

  println!("{}", n);
}

fn start_num(s: &str, i: usize) -> Option<usize> {
  let sb = s.as_bytes();
  sb[i]
    .is_ascii_digit()
    .then_some((sb[i] - b'0') as usize)
    .or(
      DIGIT_STR
        .iter()
        .enumerate()
        .find(|(_, &n)| s[i..].starts_with(n))
        .map(|(j, _)| j + 1),
    )
}
