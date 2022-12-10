#![feature(test)]
#![feature(iter_array_chunks)]

mod utils;

extern crate test;

use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = utils::get_puzzle_input(3);
}

const LOWER_A: u8 = 'a' as u8;
const UPPER_A: u8 = 'a' as u8;

type PriorityArray = [bool; 256];

pub fn main() {
  part1();
  part2::<3>();
}

fn part1() {
    let mut sum = 0u32;
    for line in INPUT.lines() {
        let bytes = line.as_bytes();
        let (left, right) = bytes.split_at(bytes.len() / 2);
        let mut priorities: PriorityArray = [false; 256];
        for &char in left {
          priorities[char as usize] = true;
        }
        for &char in right {
          if priorities[char as usize] {
            sum += char_to_priority(char) as u32;
            break;
          }
        }
    }

    println!("Part 1 Sum: {}", sum);
}

fn part2<const C: usize>() {
  let mut sum: u32 = 0;
  for lines in INPUT.lines().array_chunks::<C>() {
    let mut items = [[false; C]; 256]; 
    for (idx, line) in lines.iter().enumerate() {
      for c in line.as_bytes() {
        items[*c as usize][idx] = true;
      }
    }

    for (idx, &vals) in items.iter().enumerate() {
      if vals.iter().all(|v| *v) {
        sum += char_to_priority(idx as u8) as u32;
        break
      }
    }
  }

  println!("Part 2 sum: {}", sum);
}

fn char_to_priority(char: u8) -> u8 {
    if char >= 'a' as u8 {
        char - 'a' as u8 + 1
    } else {
        (char - 'A' as u8) + 27
    }
}

