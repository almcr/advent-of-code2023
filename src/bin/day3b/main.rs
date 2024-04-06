fn main() {
  let map: Vec<&str> = include_str!("input").lines().collect();
  let mut nums = Vec::with_capacity(2);

  println!(
    "{}",
    map
      .iter()
      .enumerate()
      .map(|(i, &line)| {
        line
          .chars()
          .enumerate()
          .filter(|(_, c)| *c == '*')
          // find digits around *, parse them
          // and add the to nums
          .filter_map(|(pos, _)| {
            nums.clear();
            let range = pos.saturating_sub(1)..=pos.saturating_add(1);
            [
              (Some(&line), range.clone()),
              (map.get(i.wrapping_sub(1)), range.clone()), // prev line
              (map.get(i + 1), range.clone()),             // next line
            ]
            .into_iter()
            .for_each(|(l, r)| {
              r.for_each(|d| {
                if l.unwrap().chars().nth(d).unwrap().is_ascii_digit() {
                  nums.push(parse_num(l.unwrap(), d))
                }
              })
            });
            // dedup is needed when digits of the same number are neighbors of *
            nums.dedup();
            (nums.len() == 2).then(|| nums.iter().product::<usize>())
          })
          .sum::<usize>()
      })
      .sum::<usize>()
  );
}

// parse a number from i by scanning back and forward
fn parse_num(s: &str, i: usize) -> usize {
  let lb = s[..i].rfind(|c: char| !c.is_ascii_digit()).map_or(0, |p| p + 1);
  let ub = s[i..].find(|c: char| !c.is_ascii_digit()).map_or(s.len(), |p| i + p);
  s[lb..ub].parse::<usize>().unwrap()
}
