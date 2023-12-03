pub struct Day02 {}

static RED: u32 = 12;
static GREEN: u32 = 13;
static BLUE: u32 = 14;

impl Day02 {
  fn process_game_part_1(game: &str) -> bool {
    let sets: Vec<&str> = game.split(&[':', ';'][..]).map(|s| &s[1..]).collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in sets.into_iter().skip(1) {
      let cubes: Vec<&str> = set.split(", ").collect();
      for colour in cubes {
        let check: Vec<&str> = colour.split_ascii_whitespace().collect();
        let num = check[0].parse::<u32>().unwrap_or(0);
        match check[1] {
          "red" => red = num,
          "green" => green = num,
          "blue" => blue = num,
          _ => (),
        }
        if !(red <= RED && green <= GREEN && blue <= BLUE) {
          return false;
        }
      }
    }

    true
  }

  pub fn part_1(input: &str) -> usize {
    let games: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for (i, game) in games.into_iter().enumerate() {
      if Self::process_game_part_1(game) {
        sum += i + 1;
      }
    }

    sum
  }

  fn process_game_part_2(game: &str) -> usize {
    let sets: Vec<&str> = game.split(&[':', ';'][..]).map(|s| &s[1..]).collect();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in sets.into_iter().skip(1) {
      let cubes: Vec<&str> = set.split(", ").collect();
      for colour in cubes {
        let check: Vec<&str> = colour.split_ascii_whitespace().collect();
        let num = check[0].parse::<usize>().unwrap_or(0);
        match check[1] {
          "red" => red = red.max(num),
          "green" => green = green.max(num),
          "blue" => blue = blue.max(num),
          _ => (),
        }
      }
    }

    red * green * blue
  }

  pub fn part_2(input: &str) -> usize {
    let games: Vec<&str> = input.lines().collect();
    let mut sum = 0;

    for game in games {
      sum += Self::process_game_part_2(game);
    }

    sum
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::{self, Read};

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/02/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 8;

    assert_eq!(Day02::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/02/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 2286;

    assert_eq!(Day02::part_2(&input), result);

    Ok(())
  }
}
