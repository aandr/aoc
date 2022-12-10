#![feature(test)]

mod utils;

extern crate test;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INPUT: String = utils::get_puzzle_input(5);
}

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

type Stack = Vec<Vec<char>>;
type Mover = fn(&mut Stack, &Move) -> ();

pub fn main() {
    run(in_order_mover);
    run(reverse_mover);
}

fn in_order_mover(stack: &mut Stack, mv: &Move) {
    for _ in 0..mv.count {
        let item = stack[mv.from].pop().unwrap();
        stack[mv.to].push(item);
    }
}

fn reverse_mover(stack: &mut Stack, mv: &Move) {
    let from_len = stack[mv.from].len();
    for idx in (from_len - mv.count)..from_len {
        let item = stack[mv.from][idx];
        stack[mv.to].push(item);
    }
    stack[mv.from].truncate(from_len - mv.count);
}

fn run(mover: Mover) {
    let mut stack = parse_stack(&INPUT);
    let moves = parse_moves(&INPUT);

    for mv in moves {
        mover(&mut stack, &mv)
    }

    println!("Result: {}", get_message(&stack));
}

fn parse_stack(input: &str) -> Stack {
    let mut lines: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();
    let num_stacks = (lines.pop().unwrap().len() as f32 / 4.).ceil() as usize;
    let mut stacks = vec![Vec::with_capacity(lines.len()); num_stacks];

    for &line in lines.iter().rev() {
        for idx in 0..num_stacks {
            let offset = idx * 4 + 1;
            if line.len() > offset {
                match line.as_bytes()[offset] as char {
                    ' ' => {}
                    c => stacks[idx].push(c),
                }
            }
        }
    }

    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    let move_re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let mut out = vec![];

    for line in input.lines().skip_while(|line| !line.is_empty()).skip(1) {
        let c = move_re.captures(line).unwrap();
        out.push(Move {
            count: c[1].parse().unwrap(),
            from: c[2].parse::<usize>().unwrap() - 1,
            to: c[3].parse::<usize>().unwrap() - 1,
        });
    }

    out
}

fn get_message(stack: &Vec<Vec<char>>) -> String {
    stack.iter().flat_map(|s| s.last()).collect::<String>()
}

#[bench]
fn part1_bench(b: &mut test::Bencher) {
    b.iter(|| run(in_order_mover));
}

#[bench]
fn part2_bench(b: &mut test::Bencher) {
    b.iter(|| run(reverse_mover));
}

#[bench]
fn parse_stack_bench(b: &mut test::Bencher) {
    b.iter(|| parse_stack(&INPUT));
}

#[bench]
fn parse_moves_bench(b: &mut test::Bencher) {
    b.iter(|| parse_stack(&INPUT));
}
