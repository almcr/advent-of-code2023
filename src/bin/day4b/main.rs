#![feature(iter_array_chunks)]

fn main() {
  let input = include_str!("input");
  let col = input.chars().position(|c| c == ':').unwrap();
  let sep = input.chars().position(|c| c == '|').unwrap();
  let mut copies = vec![1u32; 256];
  let s = input
    .lines()
    .enumerate()
    .map(|(i, card)| {
      let (winning, numbers) = (&card[col + 1..sep], &card[sep + 1..]);
      let count = (0..numbers.len())
        .array_chunks()
        .map(|[a, _, b]| &numbers[a..=b])
        .filter(|n| winning.contains(*n))
        .count();
      let n = copies[i];
      (i + 1..=i + count).for_each(|c| copies[c] += n);
      n
    })
    .sum::<u32>();
  println!("{}", s);
}
