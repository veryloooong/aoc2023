use std::collections::HashSet;

pub struct Day03 {}

struct Number {
  number: usize,
  x: usize,
  y: usize,
}

impl Day03 {
  /// Loop through every character and then store indices of those part numbers. Then iterate through said indices and add numbers.
  pub fn part_1(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input
      .lines()
      .map(|line| line.chars().collect::<Vec<char>>())
      .collect();
    let mut indices = HashSet::new();
    let rows = matrix.len();
    let cols = matrix[0].len();

    for (r, row) in matrix.iter().enumerate() {
      for (c, &ch) in row.iter().enumerate() {
        if ch.is_ascii_digit() || ch == '.' {
          continue;
        } else {
          for r_idx in (r.saturating_sub(1))..(r + 2).min(rows) {
            for c_idx in (c.saturating_sub(1))..(c + 2).min(cols) {
              if !matrix[r_idx][c_idx].is_ascii_digit() {
                continue;
              } else {
                let mut c_idx = c_idx;
                while c_idx > 0 && matrix[r_idx][c_idx - 1].is_ascii_digit() {
                  c_idx -= 1;
                }
                indices.insert((r_idx, c_idx));
              }
            }
          }
        }
      }
    }

    let mut ans = 0;

    for (r_idx, c_idx) in indices.into_iter() {
      let mut s = String::new();
      let mut c_idx = c_idx;
      while c_idx < cols && matrix[r_idx][c_idx].is_ascii_digit() {
        s.push(matrix[r_idx][c_idx]);
        c_idx += 1;
      }
      ans += s.parse::<usize>().unwrap();
    }

    ans
  }

  /// The same adding, but the indices checking has been moved inside the `'add` loop. (Thank Rust for labelled loops...) Once the indices set count reaches 2 just `break 'add;` and continue.
  pub fn part_2(input: &str) -> usize {
    let matrix: Vec<Vec<char>> = input
      .lines()
      .map(|line| line.chars().collect::<Vec<_>>())
      .collect();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = 0;

    for (r, row) in matrix.iter().enumerate() {
      for (c, &ch) in row.iter().enumerate() {
        if ch != '*' {
          continue;
        } else {
          let mut indices = HashSet::new();
          'add: for r_idx in (r.saturating_sub(1))..(r + 2).min(rows) {
            for c_idx in (c.saturating_sub(1))..(c + 2).min(cols) {
              if !matrix[r_idx][c_idx].is_ascii_digit() {
                continue;
              } else {
                let mut c_idx = c_idx;
                while c_idx > 0 && matrix[r_idx][c_idx - 1].is_ascii_digit() {
                  c_idx -= 1;
                }
                indices.insert((r_idx, c_idx));
              }

              if indices.len() == 2 {
                let mut ans = 1;
                for &(r_idx, c_idx) in indices.iter() {
                  let mut s = String::new();
                  let mut c_idx = c_idx;
                  while c_idx < cols && matrix[r_idx][c_idx].is_ascii_digit() {
                    s.push(matrix[r_idx][c_idx]);
                    c_idx += 1;
                  }
                  ans *= s.parse::<usize>().unwrap();
                }
                result += ans;
                break 'add;
              }
            }
          }
        }
      }
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use std::fs::File;
  use std::io::{self, Read};

  use super::*;

  #[test]
  fn test1() -> io::Result<()> {
    let mut input_file = File::open("./input/03/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 4361;

    assert_eq!(Day03::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/03/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 467835;

    assert_eq!(Day03::part_2(&input), result);

    Ok(())
  }
}
