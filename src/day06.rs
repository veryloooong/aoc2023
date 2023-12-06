pub struct Day06 {}

impl Day06 {
  pub fn part_1(_input: &str) -> usize {
    0
  }

  pub fn part_2(_input: &str) -> usize {
    0
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

    let result = 0;

    assert_eq!(Day06::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/06/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 0;

    assert_eq!(Day06::part_2(&input), result);

    Ok(())
  }
}
