#![feature(test)]
#![feature(core_intrinsics)]

mod utils;

extern crate test;
use lazy_static::lazy_static;
use std::intrinsics::{likely, unlikely};

lazy_static! {
    static ref INPUT: String = utils::get_puzzle_input(1);
}

pub fn main() {
    let mut current_elf: u32 = 0;
    let mut top3 = [0, 0, 0];

    for row in INPUT.split("\n") {
        if unlikely(row.is_empty()) {
            insert_sort3(&mut top3, current_elf);
            current_elf = 0;
        } else {
            let Ok(row_value) = row.parse::<u32>() else {
                panic!("Failed to parse row: '{}'", row);
            };
            current_elf += row_value;
        }
    }
    insert_sort3(&mut top3, current_elf);

    println!("Max sum: {}", top3[2]);
    println!("Max 3 sum: {}", top3.iter().sum::<u32>());
}

fn insert_sort3<T: PartialOrd + Copy>(arr: &mut [T; 3], newval: T) {
    if unlikely(newval > arr[2]) {
        arr[0] = arr[1];
        arr[1] = arr[2];
        arr[2] = newval;
    } else if unlikely(newval > arr[1]) {
        arr[0] = arr[1];
        arr[1] = newval;
    } else if likely(newval > arr[0]) {
        arr[0] = newval;
    }
}

#[bench]
fn day1_bench(b: &mut test::Bencher) {
    b.iter(|| main());
}
