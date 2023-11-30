use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;

use dotenvy::dotenv;

const AOC_YEAR: &str = "2023";

// This is a utility that asks the user for a input number
// and then creates a new crate for the advent of code challenge of that day.
// Additionally it will download the puzzle input for that day and store it in the
// newly created crate, if the session cookie is provided in the AOC_SESSION env variable or in the .env file.

fn main() {
    dotenv().ok();

    let day = get_day();
    let day_dir = create_day_dir(day);
    create_cargo_toml(day, &day_dir);
    create_src(day, &day_dir);
    create_input_file(day, &day_dir);
}

fn get_day() -> u8 {
    print!("Please enter the day you want to create a crate for: ");
    io::stdout().flush().unwrap();

    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");
    day.trim().parse::<u8>().expect("Please enter a number!")
}

fn create_day_dir(day: u8) -> PathBuf {
    let day_directory_str = format!("day{:0>2}", day);
    let day_directory = PathBuf::from(&day_directory_str);
    fs::create_dir(&day_directory).expect("Could not create day directory");

    day_directory
}

fn create_cargo_toml(day: u8, day_dir: &Path) {
    let cargo_toml_path = day_dir.join("Cargo.toml");
    let cargo_toml_str = format!(
        r#"[package]
name = "day{:02}"
version = "0.1.0"
edition = "2021"

[dependencies]
aoc-utils = {{ path = "../aoc-utils" }}
"#,
        day
    );
    fs::write(&cargo_toml_path, cargo_toml_str).expect("Could not write Cargo.toml");
}

fn create_src(day: u8, day_dir: &Path) {
    let src_dir = &day_dir.join("src");
    fs::create_dir(src_dir).expect("Could not create src directory");

    let main_rs_path = src_dir.join("main.rs");
    let main_rs_str = format!(
        r#"use aoc_utils::PuzzleInput;
const DAY: u8 = {day};

fn main() {{
    let input = PuzzleInput::get_input(DAY);
    println!("A: {{}}", solve_a(&input));
    println!("B: {{}}", solve_b(&input));
}}

fn solve_a(input: &PuzzleInput) -> usize {{
    input.lines().count()
}}

fn solve_b(input: &PuzzleInput) -> usize {{
    input.lines().count()
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_no_panic() {{
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }}

    #[test]
    fn test_solve_a() {{
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 0);
    }}

    #[test]
    fn test_solve_b() {{
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 0);
    }}
}}
"#,
        day = day
    );
    fs::write(&main_rs_path, main_rs_str).expect("Could not write main.rs");
}

fn create_input_file(day: u8, day_dir: &Path) {
    let name = format!("input_day{:02}.txt", day);
    let input_file_path = day_dir.join(name);
    let mut file = fs::File::create(input_file_path).expect("Could not create input file");

    let input_content = fetch_input(day);
    let input_content = input_content.as_bytes();
    file.write_all(input_content)
        .expect("Could not write input file");
}

fn fetch_input(day: u8) -> String {
    let session = std::env::var("AOC_SESSION").ok();
    if session.is_none() {
        println!("No AOC_SESSION environment variable found. Puzzle input will not be automatically fetched. Refer to .env.sample");
        return String::new();
    }
    let session = session.unwrap();

    let url = format!("https://adventofcode.com/{AOC_YEAR}/day/{day}/input");
    println!("Fetching input from {}...", url);
    let response = ureq::get(&url)
        .set("Cookie", &format!("session={}", session))
        .call()
        .expect("Could not fetch input");

    if response.status() != 200 {
        panic!("Could not fetch input");
    }

    println!("Input fetched successfully!");
    response.into_string().expect("Could not parse input")
}
