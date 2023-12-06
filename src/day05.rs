pub struct Day05 {}

impl Day05 {
  /// Iterate through each mapping in order and then do arithmetics to find the next numbers by continuously re-assigning to `seeds`.
  pub fn part_1(input: &str) -> usize {
    let info: Vec<&str> = input.split("\n\n").collect();
    let mut seeds: Vec<usize> = info[0]
      .split_ascii_whitespace()
      .skip(1)
      .map(|x| x.parse().unwrap())
      .collect();

    for block in info.iter().skip(1) {
      let mut ranges = vec![];
      for line in block.lines().skip(1) {
        let ints: Vec<usize> = line
          .split_ascii_whitespace()
          .map(|x| x.parse::<usize>().unwrap())
          .collect();
        ranges.push(ints);
      }

      let mut mapping = vec![];
      'map: for &seed in seeds.iter() {
        for range in ranges.iter() {
          if range[1] <= seed && seed < range[1] + range[2] {
            mapping.push(seed - range[1] + range[0]);
            continue 'map;
          }
        }
        mapping.push(seed);
      }
      seeds = mapping;
    }

    seeds.iter().min().unwrap().to_owned()
  }

  /// The same basic logic as with part 1, but now assigns on ranges.
  pub fn part_2(input: &str) -> usize {
    let info: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<usize> = info[0]
      .split_ascii_whitespace()
      .skip(1)
      .map(|x| x.parse().unwrap())
      .collect();
    let mut seeds: Vec<(usize, usize)> = seeds.chunks(2).map(|x| (x[0], x[0] + x[1])).collect();

    for block in info.iter().skip(1) {
      let mut ranges = vec![];
      for line in block.lines().skip(1) {
        let ints: Vec<usize> = line
          .split_ascii_whitespace()
          .map(|x| x.parse::<usize>().unwrap())
          .collect();
        ranges.push(ints);
      }

      let mut mapping = vec![];
      'map: while let Some((start, end)) = seeds.pop() {
        for range in ranges.iter() {
          let range_start = start.max(range[1]);
          let range_end = end.min(range[1] + range[2]);
          if range_start < range_end {
            mapping.push((
              range_start - range[1] + range[0],
              range_end - range[1] + range[0],
            ));
            if start < range_start {
              mapping.push((start, range_start));
            }
            if range_end < end {
              mapping.push((range_end, end));
            }
            continue 'map;
          }
        }
        mapping.push((start, end));
      }
      println!("{:?}", &mapping);
      seeds = mapping;
    }

    // seeds.iter().min().unwrap().to_owned()
    seeds.sort_by(|a, b| a.0.cmp(&b.0));

    seeds[0].0
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::{self, Read};

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/05/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 35;

    assert_eq!(Day05::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/05/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 46;

    assert_eq!(Day05::part_2(&input), result);

    Ok(())
  }
}
