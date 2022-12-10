mod utils;

use std::ops::{RangeInclusive};

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = utils::get_puzzle_input(4);
}

pub fn main() {
  let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
  let mut full_overlap = 0;
  let mut partial_overlap = 0;

  for line in INPUT.lines() {
    let Some(c) = re.captures(line) else {
      panic!("Failed to parse line {}", line)
    };
    let a = parse_range(&c[1], &c[2]);
    let b = parse_range(&c[3], &c[4]);

    if (a.start() <= b.end() && a.end() >= b.start()) || (b.start() <= a.end() && b.end() >= a.start()) {
      partial_overlap += 1
    }

    if (a.start() <= b.start() && a.end() >= b.end()) || (b.start() <= a.start() && b.end() >= a.end()) {
        full_overlap += 1
    }
  }

  println!("Full overlap: {}", full_overlap);
  println!("Partial overlap: {}", partial_overlap);
}

fn parse_range(low: &str, high: &str) -> RangeInclusive<u32> {
  let low_num = low.parse::<u32>().unwrap();
  let high_num = high.parse::<u32>().unwrap();
  low_num..=high_num
}