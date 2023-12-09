pub struct Day09 {}

impl Day09 {
  /// Part 1 and 2 are the exact same, with the only caveat being you have to call `rev()` on each sequence in part 2. Use `slice::windows()` for creating the next sequence.
  pub fn part_1(input: &str) -> isize {
    let sequences: Vec<&str> = input.lines().collect();
    let mut ans = 0;

    for sequence in sequences.iter() {
      let sequence: Vec<isize> = sequence
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
      let mut recurse = vec![sequence];
      let mut next;
      loop {
        let new_sequence = recurse
          .last()
          .unwrap()
          .windows(2)
          .map(|x| x[1] - x[0])
          .collect::<Vec<_>>();
        if new_sequence.windows(2).all(|x| x[0] == x[1]) {
          next = new_sequence[0];
          break;
        } else {
          recurse.push(new_sequence);
        }
      }

      for &num in recurse.iter().rev().map(|x| x.last().unwrap()) {
        next += num;
      }

      ans += next;
    }

    ans
  }

  pub fn part_2(input: &str) -> isize {
    let sequences: Vec<&str> = input.lines().collect();
    let mut ans = 0;

    for sequence in sequences.iter() {
      let sequence: Vec<isize> = sequence
        .split_whitespace()
        .rev()
        .map(|x| x.parse().unwrap())
        .collect();
      let mut recurse = vec![sequence];
      let mut next;
      loop {
        let new_sequence = recurse
          .last()
          .unwrap()
          .windows(2)
          .map(|x| x[1] - x[0])
          .collect::<Vec<_>>();
        if new_sequence.windows(2).all(|x| x[0] == x[1]) {
          next = new_sequence[0];
          break;
        } else {
          recurse.push(new_sequence);
        }
      }

      for &num in recurse.iter().rev().map(|x| x.last().unwrap()) {
        next += num;
      }

      ans += next;
    }

    ans
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::{self, Read};

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/09/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 114;

    assert_eq!(Day09::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/09/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 2;

    assert_eq!(Day09::part_2(&input), result);

    Ok(())
  }
}
