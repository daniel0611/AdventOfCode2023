use std::vec;

use aoc_utils::PuzzleInput;
const DAY: u8 = 1;
const SPELLED_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|d| d.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .map(|digits| digits[0] * 10 + digits[digits.len() - 1])
        .sum()
}

fn get_digits(line_original: String) -> Vec<usize> {
    let mut line = line_original.clone().to_lowercase();
    let mut digits = vec![];

    while !line.is_empty() {
        let mut found_spelled_digit = false;
        for (i, spelled_digit) in SPELLED_DIGITS.iter().enumerate() {
            if line.starts_with(spelled_digit) {
                digits.push(i);
                line = line[spelled_digit.len()..].to_string();
                found_spelled_digit = true;
                break;
            }
            // Handle overlapping spelled digits
            if line.starts_with(&spelled_digit[1..]) {
                digits.push(i);
                line = line[spelled_digit[1..].len()..].to_string();
                found_spelled_digit = true;
                break;
            }
        }
        if found_spelled_digit {
            continue;
        }

        let c = line.chars().next().unwrap();
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap() as usize);
        }

        line = line[1..].to_string();
    }
    println!("{} {:?}", line_original, digits);

    digits
}

fn solve_b(input: &PuzzleInput) -> usize {
    input
        .lines()
        .map(get_digits)
        .map(|digits| digits[0] * 10 + digits[digits.len() - 1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_A: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT_B: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT_A)), 142);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT_B)), 281);
    }
}
