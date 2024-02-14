fn main() {
  let s: usize = include_str!("input")
    .lines()
    .map(|l| game_power_num(l.split_once(": ").unwrap().1))
    .sum();
  println!("{}", s);
}

fn game_power_num(game: &str) -> usize {
  let (mut max_red, mut max_green, mut max_blue) = (0usize, 0, 0);
  game.split("; ").for_each(|cubes| {
    cubes.split(", ").for_each(|cube| {
      let (n, color) = cube.split_once(' ').unwrap();
      let n = n.parse::<usize>().unwrap();
      match color {
        "red" => max_red = max_red.max(n),
        "green" => max_green = max_green.max(n),
        "blue" => max_blue = max_blue.max(n),
        _ => unreachable!(),
      }
    });
  });
  max_red * max_green * max_blue
}
