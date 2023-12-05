use std::collections::HashSet;

pub struct Day04 {}

fn process_game(game: &str) -> usize {
  let tokens: Vec<&str> = game.split_ascii_whitespace().skip(2).collect();
  let mut count: usize = 0;
  let nums: Vec<Vec<&str>> = tokens.split(|&x| x == "|").map(|x| x.to_vec()).collect();

  let winning_numbers: HashSet<&str> = nums[0].iter().cloned().collect();
  for num in nums[1].iter() {
    if winning_numbers.contains(num) {
      count += 1;
    }
  }

  count
}

impl Day04 {
  /// Put each games' winning numbers into a set, count the number of winning numbers in my deck, then return `2**(count-1)` for that game.
  pub fn part_1(input: &str) -> usize {
    let games: Vec<&str> = input.lines().collect();
    let mut ans = 0;

    for game in games.into_iter() {
      let count = process_game(game);
      ans += if count == 0 {
        0
      } else {
        2usize.pow(count as u32 - 1)
      };
    }

    ans
  }

  /// Keep a count array and increment the card count.
  pub fn part_2(input: &str) -> usize {
    let games: Vec<&str> = input.lines().collect();
    let mut counts = vec![1usize; games.len()];

    for (card_num, game) in games.iter().enumerate() {
      let card_amount = counts[card_num];
      let count = process_game(game);
      for next_card in counts.iter_mut().skip(card_num + 1).take(count) {
        *next_card += card_amount;
      }
    }

    counts.iter().sum()
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::{self, Read};

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/04/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 13;

    assert_eq!(Day04::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/04/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 30;

    assert_eq!(Day04::part_2(&input), result);

    Ok(())
  }
}
