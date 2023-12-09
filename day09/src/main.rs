use aoc_utils::PuzzleInput;
const DAY: u8 = 9;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_numbers(line: String) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

fn calculate_diffs(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut difference_vecs: Vec<Vec<i64>> = Vec::new();
    difference_vecs.push(history.clone());

    while !difference_vecs.last().unwrap().iter().all(|v| *v == 0) {
        let mut difference_vec = Vec::new();
        let previous_diffs = difference_vecs.last().unwrap();
        for i in 0..previous_diffs.len() - 1 {
            difference_vec.push(previous_diffs[i + 1] - previous_diffs[i]);
        }
        difference_vecs.push(difference_vec);
    }

    difference_vecs
}

fn predict_next_value(history: Vec<i64>) -> i64 {
    let mut difference_vecs: Vec<Vec<i64>> = calculate_diffs(history);

    // Compute the next difference for each order
    difference_vecs.last_mut().unwrap().push(0);
    for i in (0..difference_vecs.len() - 1).rev() {
        let new_value = {
            let current_history = &difference_vecs[i];
            let higher_history = &difference_vecs[i + 1];

            current_history.last().unwrap() + higher_history.last().unwrap()
        };

        difference_vecs[i].push(new_value);
    }

    *difference_vecs.first().unwrap().last().unwrap()
}

fn predict_previous_value(history: Vec<i64>) -> i64 {
    let mut difference_vecs: Vec<Vec<i64>> = calculate_diffs(history);

    difference_vecs.last_mut().unwrap().push(0);
    for i in (0..difference_vecs.len() - 1).rev() {
        let new_value = {
            let current_history = &difference_vecs[i];
            let higher_history = &difference_vecs[i + 1];

            current_history.first().unwrap() - higher_history.first().unwrap()
        };

        difference_vecs[i].insert(0, new_value);
    }

    *difference_vecs.first().unwrap().first().unwrap()
}

fn solve_a(input: &PuzzleInput) -> i64 {
    input
        .lines()
        .map(|line| parse_numbers(line.to_string()))
        .map(predict_next_value)
        .sum()
}

fn solve_b(input: &PuzzleInput) -> i64 {
    input
        .lines()
        .map(|line| parse_numbers(line.to_string()))
        .map(predict_previous_value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 114);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 2);
    }
}
