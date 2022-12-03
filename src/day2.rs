#![feature(test)]
#![feature(core_intrinsics)]

mod utils;

extern crate test;
use core::panic;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = utils::get_puzzle_input(2);
}


pub fn main() {
  let mut score = 0u32;
  for line in INPUT.split("\n") {
    if line == "A Y" || line == "B Z" || line == "C X" {
      score += 6
    }
    if line == "A X" || line == "B Y" || line == "C Z" {
      score += 3
    }
    match line.chars().last() {
      Some('X') => score += 1,
      Some('Y') => score += 2,
      Some('Z') => score += 3,
      Some(_) => panic!(),
      None => {}
    }
  }

  println!("Score: {}", score)
}

#[derive(PartialEq)]
#[repr(u32)]
enum Move {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

impl Move {
  fn wins_over(&self, other: &Move) -> bool {
    match (self, other) {
      (Move::Rock, Move::Scissors) => true,
      (Move::Paper, Move::Rock) => true,
      (Move::Scissors, Move::Paper) => true,
      (_, _) => false
    }
  }

  fn parse(input: char) -> Move {
    match input {
      'A' | 'X' => Move::Rock,
      'B' | 'Y' => Move::Paper,
      'C' | 'Z' => Move::Scissors,
      _ => panic!("Unknown character {}", input)
    }
  }

}