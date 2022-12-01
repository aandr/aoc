use std::path::Path;

use reqwest::header::COOKIE;

pub fn get_puzzle_input(day: u8) -> String {
    let cache_path = format!("./.cache/{}.txt", &day);
    let cache_path = Path::new(&cache_path);
    if cache_path.exists() {
        std::fs::read_to_string(cache_path).expect("Failed to read cache file")
    } else {
        let text = download_puzzle_input(day);
        std::fs::write(cache_path, &text).expect("Failed to cache problem data");
        text
    }
}

fn download_puzzle_input(day: u8) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);

    client
        .get(url)
        .header(COOKIE, get_aoc_cookie().trim())
        .send()
        .expect("Failed to download problem data")
        .text()
        .expect("Failed to parse problem data download")
}

fn get_aoc_cookie() -> String {
    std::fs::read_to_string("./.aoc_session")
        .expect("Please save your Advent of Code cookie to .aoc_session")
}
