pub struct Day06 {}

impl Day06 {
  /// Since the numbers are smaller in part 1, I just run a loop through the numbers.
  pub fn part_1(input: &str) -> usize {
    let info: Vec<_> = input.lines().collect();
    let times: Vec<usize> = info[0]
      .split_ascii_whitespace()
      .skip(1)
      .map(|x| x.parse().unwrap())
      .collect();
    let distances: Vec<usize> = info[1]
      .split_ascii_whitespace()
      .skip(1)
      .map(|x| x.parse().unwrap())
      .collect();

    let mut ans = 1;

    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
      let mut count = 0;
      for hold in 1..time {
        if hold * (time - hold) > distance {
          count += 1;
        }
      }
      ans *= count;
    }

    ans
  }

  /// Essentially we're solving a quadratic function here.
  ///
  /// We're solving `x * (time - x) >= distance`, which is `x^2 - time * x + distance <= 0`. For the first time in forever, the formulae are helpful...
  pub fn part_2(input: &str) -> usize {
    let info: Vec<_> = input.lines().collect();
    let time: usize = info[0]
      .split_ascii_whitespace()
      .skip(1)
      .flat_map(|x| x.chars())
      .collect::<String>()
      .parse()
      .unwrap();
    let distance: usize = info[1]
      .split_ascii_whitespace()
      .skip(1)
      .flat_map(|x| x.chars())
      .collect::<String>()
      .parse()
      .unwrap();

    let discriminant = (time * time - 4 * distance) as f64;

    let x_1 = (time as f64 - discriminant.sqrt()) / 2.0;
    let x_2 = (time as f64 + discriminant.sqrt()) / 2.0;

    (x_2 - x_1) as usize
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::{self, Read};

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/06/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 288;

    assert_eq!(Day06::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/06/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 71503;

    assert_eq!(Day06::part_2(&input), result);

    Ok(())
  }
}
