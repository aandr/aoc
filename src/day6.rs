#![feature(test)]

mod utils;

extern crate test;
use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: String = utils::get_puzzle_input(6);
}

pub fn main() {
    find_distinct(4);
    find_distinct(14);
}

fn find_distinct(num_distinct: usize) {
    let bytes = INPUT.as_bytes();
    let mut counts = [false; 256];

    'outer: for (idx, window) in bytes.windows(num_distinct).enumerate() {
        for i in 0..num_distinct {
            match counts[window[i] as usize] {
                true => {
                    counts.fill(false);
                    continue 'outer;
                }
                false => counts[window[i] as usize] = true,
            }
        }

        println!("Distinct {}: {}", num_distinct, idx + num_distinct);
        return;
    }
}

#[bench]
fn find_distinct_bench(b: &mut test::Bencher) {
    b.iter(|| find_distinct(14));
}
