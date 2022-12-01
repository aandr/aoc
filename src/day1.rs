#![feature(test)]
#![feature(core_intrinsics)]

mod utils;

extern crate test;
use lazy_static::lazy_static;
use std::{
    intrinsics::{likely, unlikely},
    thread::current,
};

lazy_static! {
    static ref INPUT: String = utils::get_puzzle_input(1);
}

pub fn main() {
    let mut current_elf: u32 = 0;
    // Keep a running list of the top 4, whereas the last element will be swapped by the latest elf.
    let mut top4 = [0, 0, 0, 0];

    for row in INPUT.split('\n') {
        if unlikely(row.is_empty()) {
            top4[3] = current_elf;
            sort4(&mut top4);
            current_elf = 0;
        } else {
            let Ok(row_value) = row.parse::<u32>() else {
                panic!("Failed to parse row: '{}'", row);
            };
            current_elf += row_value;
        }
    }
    println!("Max sum: {}", top4[0]);
    println!("Max 3 sum: {}", top4[0] + top4[1] + top4[2]);
}

fn sort4<T: PartialOrd>(arr: &mut [T; 4]) {
    cond_swap(arr, 0, 2);
    cond_swap(arr, 1, 3);
    cond_swap(arr, 0, 1);
    cond_swap(arr, 2, 3);
    cond_swap(arr, 1, 2);
}

#[inline]
fn cond_swap<T: PartialOrd, const S: usize>(arr: &mut [T; S], x: usize, y: usize) {
    // Branch prediction hint.
    if likely(arr[x] < arr[y]) {
        arr.swap(x, y);
    }
}

#[bench]
fn day1_bench(b: &mut test::Bencher) {
    b.iter(|| main());
}
