#![allow(dead_code)]
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod template;

use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;

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

fn day_03() -> io::Result<()> {
  let mut input_file = File::open("./input/03/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 03 part 1: {}", Day03::part_1(&input));
  println!("Day 03 part 2: {}", Day03::part_2(&input));

  Ok(())
}

fn day_04() -> io::Result<()> {
  let mut input_file = File::open("./input/04/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 04 part 1: {}", Day04::part_1(&input));
  println!("Day 04 part 2: {}", Day04::part_2(&input));

  Ok(())
}

fn day_05() -> io::Result<()> {
  let mut input_file = File::open("./input/05/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 05 part 1: {}", Day05::part_1(&input));
  println!("Day 05 part 2: {}", Day05::part_2(&input));

  Ok(())
}

fn day_06() -> io::Result<()> {
  let mut input_file = File::open("./input/06/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 06 part 1: {}", Day06::part_1(&input));
  println!("Day 06 part 2: {}", Day06::part_2(&input));

  Ok(())
}

fn day_07() -> io::Result<()> {
  let mut input_file = File::open("./input/07/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 07 part 1: {}", Day07::part_1(&input));
  println!("Day 07 part 2: {}", Day07::part_2(&input));

  Ok(())
}

fn day_08() -> io::Result<()> {
  let mut input_file = File::open("./input/08/input.txt")?;
  let mut input = String::new();

  input_file.read_to_string(&mut input)?;

  println!("Day 08 part 1: {}", Day08::part_1(&input));
  println!("Day 08 part 2: {}", Day08::part_2(&input));

  Ok(())
}

/// The run to get each day's answer. I could use a macro to generate a script because DRY, but I can't be bothered...
fn main() -> io::Result<()> {
  // day_01()?;
  // day_02()?;
  // day_03()?;
  // day_04()?;
  // day_05()?;
  // day_06()?;
  // day_07()?;
  day_08()?;

  Ok(())
}
