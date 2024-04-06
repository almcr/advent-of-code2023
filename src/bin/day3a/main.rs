fn main() {
  let map: Vec<&str> = include_str!("input").lines().collect();

  println!(
    "{}",
    map
      .iter()
      .enumerate()
      .map(|(i, &line)| {
        line
          .chars()
          .enumerate()
          // begining of numbers
          .filter(|(j, c)| {
            c.is_ascii_digit()
              && line
                .chars()
                .nth(j.wrapping_sub(1))
                .map_or(true, |c| !c.is_ascii_digit())
          })
          // parse and compute number length
          .map(|(j, _)| {
            println!("{}", j);
            let p = line
              .chars()
              .skip(j)
              .position(|c| !c.is_ascii_digit())
              .map_or(line.len(), |p| p + j);
            (j, p, line[j..p].parse::<usize>().unwrap())
          })
          .filter(|(j, p, _)| {
            let range = *j as i32 - 1..*p as i32 + 1;
            let pass = [
              (Some(&line), range.clone()),
              (map.get(i.wrapping_sub(1)), range.clone()), // prev line
              (map.get(i + 1), range.clone()),             // next line
            ]
            .into_iter()
            .any(|(l, mut r)| {
              l.map_or(false, |l| {
                r.any(|d| {
                  l.chars()
                    .nth(d as usize)
                    .map_or(false, |c| c != '.' && c.is_ascii_punctuation())
                })
              })
            });
            pass
          })
          .map(|(_, _, n)| n)
          .sum::<usize>()
      })
      .sum::<usize>()
  );
}
