pub struct Day01 {}

static LOOKUP: [(&str, u32); 9] = [
  ("one", 1),
  ("two", 2),
  ("three", 3),
  ("four", 4),
  ("five", 5),
  ("six", 6),
  ("seven", 7),
  ("eight", 8),
  ("nine", 9),
];

impl Day01 {
  fn filter_digits(string: &str) -> u32 {
    let arr: Vec<u32> = string.chars().filter_map(|c| c.to_digit(10)).collect();

    arr[0] * 10 + arr.last().unwrap()
  }

  /// Beautiful, functional, one run throughout the entire file, resulting in `O(n)`.
  pub fn part_1(input: &str) -> usize {
    let arr: Vec<&str> = input.split_ascii_whitespace().collect();

    arr.iter().map(|s| Self::filter_digits(s) as usize).sum()
  }

  /// Classic imperative approach. Does a total of `10 * input.len()` checks, which is still `O(n)`. But not as clean.
  ///
  /// Optimisations include:
  /// - Skipping forward `lookup.0` indices (accounting for cases like `oneight` and `fiveight`).
  /// - Two pointers running to close down on each line. Still `O(n)` but can result in at most half the checks.
  pub fn part_2(input: &str) -> usize {
    let arr: Vec<&str> = input.split_ascii_whitespace().collect();

    let mut sum = 0;

    for line in arr.into_iter() {
      let mut nums = vec![];

      for (i, c) in line.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
          nums.push(d);
          continue;
        } else {
          for (string, value) in LOOKUP.iter() {
            if i + string.len() > line.len() {
              continue;
            } else if &&line[i..i + string.len()] == string {
              nums.push(*value);
              break;
            }
          }
        }
      }

      sum += (nums[0] * 10 + nums.last().unwrap()) as usize;
    }

    sum
  }
}

#[cfg(test)]
mod tests {
  use std::{
    fs::File,
    io::{self, Read},
  };

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/01/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    assert_eq!(Day01::part_1(&input), 142);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/01/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    assert_eq!(Day01::part_2(&input), 281);

    Ok(())
  }
}
