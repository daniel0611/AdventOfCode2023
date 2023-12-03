use aoc_utils::PuzzleInput;
const DAY: u8 = 3;

struct Number {
    row: usize,
    start_col: usize,
    end_col: usize,
    value: usize,
}

struct Map {
    map: Vec<Vec<char>>,
    numbers: Vec<Number>,
}

impl Map {
    fn parse(input: &PuzzleInput) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut numbers = Vec::new();

        for (row, line) in map.iter().enumerate() {
            let mut col = 0;
            while col < line.len() {
                if line[col].is_ascii_digit() {
                    let start_col = col;

                    while col < line.len() && line[col].is_ascii_digit() {
                        col += 1;
                    }
                    let end_col = col;

                    numbers.push(Number {
                        row,
                        start_col,
                        end_col,
                        value: line[start_col..end_col]
                            .iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap(),
                    });
                } else {
                    col += 1;
                }
            }
        }

        Self { map, numbers }
    }

    fn get_numbers_adjacent_to_symbols(&self) -> Vec<&Number> {
        self.numbers
            .iter()
            .filter(|number| self.number_is_adjacent_to_symbol(number))
            .collect()
    }

    fn number_is_adjacent_to_symbol(&self, number: &Number) -> bool {
        for col in number.start_col..number.end_col {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let x = col as i32 + dx;
                    let y = number.row as i32 + dy;

                    if x < 0 || y < 0 {
                        continue;
                    }

                    let x = x as usize;
                    let y = y as usize;

                    if y >= self.map.len() || x >= self.map[y].len() {
                        continue;
                    }

                    if self.map[y][x] != '.' && !self.map[y][x].is_ascii_digit() {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn get_adjacent_numbers_to_symbol(&self, symbol: char) -> Vec<Vec<&Number>> {
        let mut symbol_positions = vec![];
        for (row, line) in self.map.iter().enumerate() {
            for (col, c) in line.iter().enumerate() {
                if *c == symbol {
                    symbol_positions.push((row, col));
                }
            }
        }

        let mut adjacent_numbers = vec![];
        for (row, col) in symbol_positions {
            let mut numbers = vec![];
            for number in &self.numbers {
                if number.row == row || number.row == row + 1 || (row > 0 && number.row == row - 1)
                {
                    if number.start_col == 0 {
                        if (number.start_col..=number.end_col).contains(&col) {
                            numbers.push(number);
                        }
                    } else {
                        if (number.start_col - 1..=number.end_col).contains(&col) {
                            numbers.push(number);
                        }
                    }
                }
            }
            adjacent_numbers.push(numbers);
        }

        adjacent_numbers
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    Map::parse(input)
        .get_numbers_adjacent_to_symbols()
        .iter()
        .map(|number| number.value)
        .sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    Map::parse(input)
        .get_adjacent_numbers_to_symbol('*')
        .iter()
        .filter(|adj_numbers| adj_numbers.len() == 2)
        .map(|numbers| numbers[0].value * numbers[1].value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 4361);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 467835);
    }
}
