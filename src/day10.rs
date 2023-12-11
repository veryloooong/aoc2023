pub struct Day10 {}

fn calc_next(
  current_pos: (usize, usize),
  next: (isize, isize),
  bounds: (usize, usize),
) -> Option<(usize, usize)> {
  let new_pos = (
    current_pos.0 as isize + next.0,
    current_pos.1 as isize + next.1,
  );

  if new_pos.0 < 0
    || new_pos.1 < 0
    || new_pos.0 >= bounds.0 as isize
    || new_pos.1 >= bounds.1 as isize
  {
    None
  } else {
    Some((new_pos.0 as usize, new_pos.1 as usize))
  }
}

impl Day10 {
  /// We find the length of the entire loop, then divide the loop length by 2 to find the furthest point from `S`.
  pub fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
      .lines()
      .map(|x| x.chars().collect::<Vec<_>>())
      .collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let bounds = (rows, cols);
    let mut start_row = 0;
    let mut start_col = 0;

    for (i, row) in grid.iter().enumerate() {
      for (j, &ch) in row.iter().enumerate() {
        if ch == 'S' {
          (start_row, start_col) = (i, j);
          break;
        }
      }
    }

    let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_direction = 0;
    let mut current_pos = (start_row, start_col);
    let mut len = 0;

    for dir in 0..4 {
      if let Some(new_pos) = calc_next(current_pos, moves[dir], bounds) {
        let (x, y) = new_pos;
        let ch = grid[x][y];
        if (dir == 0 && !"7|F".contains(ch))
          || (dir == 1 && !"J-7".contains(ch))
          || (dir == 2 && !"L|J".contains(ch))
          || (dir == 3 && !"F-L".contains(ch))
        {
          continue;
        }
        current_pos = new_pos;
        len += 1;
        current_direction = dir;
        break;
      };
    }

    loop {
      let (x, y) = current_pos;
      match grid[x][y] {
        'S' => break,
        '-' | '|' => {
          if let Some(new_pos) = calc_next(current_pos, moves[current_direction], bounds) {
            current_pos = new_pos;
          }
        }
        'J' => {
          current_direction = if current_direction == 1 { 0 } else { 3 };

          if let Some(new_pos) = calc_next(current_pos, moves[current_direction], bounds) {
            current_pos = new_pos;
          }
        }
        'L' => {
          current_direction = if current_direction == 3 { 0 } else { 1 };

          if let Some(new_pos) = calc_next(current_pos, moves[current_direction], bounds) {
            current_pos = new_pos;
          }
        }
        'F' => {
          current_direction = if current_direction == 0 { 1 } else { 2 };

          if let Some(new_pos) = calc_next(current_pos, moves[current_direction], bounds) {
            current_pos = new_pos;
          }
        }
        '7' => {
          current_direction = if current_direction == 0 { 3 } else { 2 };

          if let Some(new_pos) = calc_next(current_pos, moves[current_direction], bounds) {
            current_pos = new_pos;
          }
        }
        _ => (),
      }
      len += 1;
    }

    len / 2
  }

  // TODO: finish part 2
  /// It appears part 2 requires from raycasting magic. Will come back later.
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
    let mut input_file = File::open("./input/10/test1.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 4;

    assert_eq!(Day10::part_1(&input), result);

    Ok(())
  }

  #[test]
  fn test2() -> io::Result<()> {
    let mut input_file = File::open("./input/10/test2.txt")?;
    let mut input: String = String::new();

    input_file.read_to_string(&mut input)?;

    let result = 0;

    assert_eq!(Day10::part_2(&input), result);

    Ok(())
  }
}
