#![feature(test)]
#![feature(slice_swap_unchecked)]

use std::mem::swap;

use lazy_static::lazy_static;
use subtle::ConditionallySelectable;

mod utils;

lazy_static! {
  static ref INPUT: String = utils::get_puzzle_input(1);
}

pub fn main() {
    let mut sums = vec![0];

    for row in INPUT.split('\n') {
        if row.is_empty() {
            sums.push(0)
        } else {
            let Ok(row_value) = row.parse::<i32>() else {
                panic!("Failed to parse row: '{}'", row);
            };
            *sums.last_mut().unwrap() += row_value;
        }
    }

    // The last 3 elements contains the top 3. Element 0 is a buffer.
    let mut top4 = [0, 0, 0, 0];

    for sum in sums {
      top4[0] = sum;
      // Inspired by LLVM's sort4_branchless.
      cond_swap(&mut top4, 0, 2);
      cond_swap(&mut top4, 1, 3);
      cond_swap(&mut top4, 0, 1);
      cond_swap(&mut top4, 2, 3);
      cond_swap(&mut top4, 1, 2);
    }

    println!("Max sum: {}", top4[3]);
    println!("Max 3 sum: {}", top4[1] + top4[2] + top4[3]);
}

#[inline]
fn cond_swap(arr: &mut [i32; 4], x: usize, y: usize) {
  if arr[x] > arr[y] {
    arr.swap(x, y);
  }
}

extern crate test;
#[bench]
fn day1_bench(b: &mut test::Bencher) {
  b.iter(|| main());
}