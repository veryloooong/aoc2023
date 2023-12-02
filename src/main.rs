#![allow(dead_code)]
mod day01;
mod day02;
mod template;

use day01::Day01;
use day02::Day02;

use std::{
  fs::File,
  io::{self, Read},
};

fn day_01() -> io::Result<()> {
  let mut input_file = File::open("./input/01/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 01 part 1: {}", Day01::part_1(&input));
  println!("Day 01 part 2: {}", Day01::part_2(&input));

  Ok(())
}

fn day_02() -> io::Result<()> {
  let mut input_file = File::open("./input/02/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 02 part 1: {}", Day02::part_1(&input));
  println!("Day 02 part 2: {}", Day02::part_2(&input));

  Ok(())
}

fn main() -> io::Result<()> {
  day_01()?;
  day_02()?;

  Ok(())
}
