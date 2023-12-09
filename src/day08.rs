use std::collections::HashMap;

pub struct Day08 {}

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

fn lcm(arr: &Vec<usize>) -> usize {
  if arr.is_empty() {
    return 0;
  }

  let mut res = arr[0];
  for &num in arr.iter().skip(1) {
    res = res * num / gcd(res, num);
  }

  res
}

impl Day08 {
  /// Simply iterate until we can find the `ZZZ` item. This is essentially a graph problem
  pub fn part_1(input: &str) -> usize {
    let things: Vec<&str> = input.lines().collect();
    let mut steps = things[0].chars().collect::<Vec<_>>();
    let mut navigation = HashMap::new();
    for map in things.iter().skip(2) {
      let insts = map.split(" = ").collect::<Vec<_>>();
      let pos = insts[0];
      let mut next = insts[1].split(", ").collect::<Vec<_>>();
      next[0] = next[0].trim_start_matches('(');
      next[1] = next[1].trim_end_matches(')');
      navigation.insert(pos, next);
    }

    let mut count = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
      count += 1;
      if steps[0] == 'L' {
        current = navigation[current][0];
      } else {
        current = navigation[current][1];
      }
      steps.rotate_left(1);
    }

    count
  }

  /// We will find a cycle for each starting token (ends with A), so all of the tokens that end with Z. Apparently the solution would be a bit more difficult, because it's more than just the LCMs of the first cycle counts. But I guess the author wanted for things not to get out of hand again?
  ///
  /// Watch [HyperNeutrino's video](https://www.youtube.com/watch?v=_nnxLcrwO_U) for a good explanation.
  pub fn part_2(input: &str) -> usize {
    let things: Vec<&str> = input.lines().collect();
    let steps = things[0].chars().collect::<Vec<_>>();
    let mut navigation = HashMap::new();
    for map in things.iter().skip(2) {
      let insts = map.split(" = ").collect::<Vec<_>>();
      let pos = insts[0];
      let mut next = insts[1].split(", ").collect::<Vec<_>>();
      next[0] = next[0].trim_start_matches('(');
      next[1] = next[1].trim_end_matches(')');
      navigation.insert(pos, next);
    }

    let starts = navigation
      .keys()
      .filter(|x| x.ends_with('A'))
      .collect::<Vec<_>>();

    let mut cycles = vec![];

    for &&current in starts.iter() {
      let mut current = current;
      let mut cycle = vec![];
      let mut count: usize = 0;
      let mut steps = steps.clone();
      let mut first_stop = "";

      loop {
        while count == 0 || !current.ends_with('Z') {
          count += 1;
          if steps[0] == 'L' {
            current = navigation[current][0];
          } else {
            current = navigation[current][1];
          }
          steps.rotate_left(1);
        }

        if first_stop.is_empty() {
          first_stop = current;
          cycle.push(count);
          count = 0;
        } else if current == first_stop {
          cycle.push(count);
          break;
        }
      }

      cycles.push(cycle[0]);
    }

    lcm(&cycles)
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::{self, Read};

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/08/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 6;

    assert_eq!(Day08::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/08/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 6;

    assert_eq!(Day08::part_2(&input), result);

    Ok(())
  }
}
