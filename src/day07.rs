use std::{cmp::Ordering, collections::HashMap};

pub struct Day07 {}

/// Hand types.
///
/// - 5-of-a-kind is 6;
/// - 4-of-a-kind is 5;
/// - Full house is 4;
/// - 3-of-a-kind is 3;
/// - 2-pair is 2;
/// - 1-pair is 1;
/// - High card is 0.
fn hand_type(hand: &str) -> usize {
  let mut hs = HashMap::new();
  for ch in hand.chars() {
    hs.entry(ch).and_modify(|c| *c += 1).or_insert(1);
  }

  match hs.len() {
    1 => 6,
    2 => {
      if hs.values().any(|&x| x == 4) {
        5
      } else {
        4
      }
    }
    3 => {
      if hs.values().any(|&x| x == 3) {
        3
      } else {
        2
      }
    }
    4 => 1,
    _ => 0,
  }
}

const fn card_value(card: char) -> usize {
  match card {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 11,
    'T' => 10,
    ch => {
      if let Some(d) = ch.to_digit(10) {
        d as usize
      } else {
        0
      }
    }
  }
}

/// For the joker cards, things behave a bit differently.
fn hand_type_joker(hand: &str) -> usize {
  let mut hs = HashMap::new();
  for ch in hand.chars() {
    hs.entry(ch).and_modify(|c| *c += 1).or_insert(1);
  }

  if hs.get(&'J').is_none() {
    hand_type(hand)
  } else {
    match hs.len() {
      // If there is a joker card, and there are 1/2 labels, then they can be coerced to the same label, resulting in a 5-of-a-kind hand.
      1 => 6,
      2 => 6,
      3 => {
        // This will always be coerced into a 4-of-a-kind.
        if hs.values().any(|&x| x == 3) {
          5
        } else {
          // If there is one J card, then it's a full house, else a 4-of-a-kind
          let c = hs.get(&'J').unwrap().to_owned();
          if c == 1 {
            4
          } else {
            5
          }
        }
      }
      4 => 3,
      _ => 1,
    }
  }
}

const fn card_value_joker(card: char) -> usize {
  match card {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' => 0,
    'T' => 10,
    ch => {
      if let Some(d) = ch.to_digit(10) {
        d as usize
      } else {
        0
      }
    }
  }
}

impl Day07 {
  pub fn part_1(input: &str) -> usize {
    let mut hands: Vec<Vec<&str>> = input
      .lines()
      .map(|x| x.split_whitespace().collect::<Vec<_>>())
      .collect();

    hands.sort_unstable_by(|a, b| match hand_type(a[0]).cmp(&hand_type(b[0])) {
      Ordering::Equal => {
        for (x, y) in a[0].chars().zip(b[0].chars()) {
          match card_value(x).cmp(&card_value(y)) {
            Ordering::Equal => continue,
            other => return other,
          };
        }
        Ordering::Equal
      }
      other => other,
    });

    let mut ans = 0;

    for (i, hand) in hands.iter().enumerate() {
      ans += (i + 1) * hand[1].parse::<usize>().unwrap();
    }

    ans
  }

  pub fn part_2(input: &str) -> usize {
    let mut hands: Vec<Vec<&str>> = input
      .lines()
      .map(|x| x.split_whitespace().collect::<Vec<_>>())
      .collect();

    hands.sort_unstable_by(
      |a, b| match hand_type_joker(a[0]).cmp(&hand_type_joker(b[0])) {
        Ordering::Equal => {
          for (x, y) in a[0].chars().zip(b[0].chars()) {
            match card_value_joker(x).cmp(&card_value_joker(y)) {
              Ordering::Equal => continue,
              other => return other,
            };
          }
          Ordering::Equal
        }
        other => other,
      },
    );

    let mut ans = 0;

    for (i, hand) in hands.iter().enumerate() {
      ans += (i + 1) * hand[1].parse::<usize>().unwrap();
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
    let mut input_file = File::open("./input/07/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 6440;

    assert_eq!(Day07::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/07/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 5905;

    assert_eq!(Day07::part_2(&input), result);

    Ok(())
  }
}
