pub struct Day11 {}

impl Day11 {
  /// I was solving part 1 and ended up solving part 2 by accident. Day 11 is MUCH easier than day 10!
  ///
  /// Basically, I find the free rows and free cols in the grid. Then for each pair of points I calculate their [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) and increase the distance every time there is a free row / column inbetween. With the `scale` arg I can control the expansion scale.
  /// - For part 1 plug `scale = 2`.
  /// - For part 2 plug `scale = 1_000_000`.
  pub fn solver(input: &str, scale: usize) -> usize {
    let grid: Vec<Vec<char>> = input
      .lines()
      .map(|x| x.chars().collect::<Vec<_>>())
      .collect();
    let cols = grid[0].len();

    let free_rows = grid
      .iter()
      .enumerate()
      .filter(|(_, r)| r.iter().all(|&x| x == '.'))
      .map(|x| x.0)
      .to_owned()
      .collect::<Vec<_>>();
    let free_cols = (0..cols)
      .filter(|&x| grid.iter().all(|r| r[x] == '.'))
      .collect::<Vec<_>>();

    let mut planets = vec![];

    for (r, row) in grid.iter().enumerate() {
      for (c, &ch) in row.iter().enumerate() {
        if ch == '#' {
          planets.push((r, c));
        }
      }
    }

    let manhattan = |a: (usize, usize),
                     b: (usize, usize),
                     free_rows: &Vec<usize>,
                     free_cols: &Vec<usize>|
     -> usize {
      let mut row_gap = a.0.abs_diff(b.0);
      let mut col_gap = a.1.abs_diff(b.1);
      for &r in free_rows.iter() {
        if a.0.min(b.0) <= r && r <= a.0.max(b.0) {
          row_gap += scale - 1;
        }
      }
      for &c in free_cols.iter() {
        if a.1.min(b.1) <= c && c <= a.1.max(b.1) {
          col_gap += scale - 1;
        }
      }

      row_gap + col_gap
    };

    let mut ans = 0;

    for i in 0..planets.len() {
      for j in i..planets.len() {
        ans += manhattan(planets[i], planets[j], &free_rows, &free_cols);
      }
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
    let mut input_file = File::open("./input/11/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 374;
    assert_eq!(Day11::solver(&input, 2), result);

    let result = 1030;
    assert_eq!(Day11::solver(&input, 10), result);

    let result = 8410;
    assert_eq!(Day11::solver(&input, 100), result);

    Ok(())
  }
}
