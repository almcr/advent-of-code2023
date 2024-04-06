#![feature(iter_array_chunks)]

fn main() {
  let input = include_str!("input");
  let col = input.chars().position(|c| c == ':').unwrap();
  let sep = input.chars().position(|c| c == '|').unwrap();
  println!(
    "{}",
    input
      .lines()
      .map(|card| {
        let (winning, numbers) = (&card[col + 1..sep], &card[sep + 1..]);
        let count = (0..numbers.len())
          .array_chunks()
          .map(|[a, _, b]| &numbers[a..=b])
          .filter(|n| winning.contains(*n))
          .count();
        2u32.pow(count as u32) >> 1
      })
      .sum::<u32>()
  );
}
