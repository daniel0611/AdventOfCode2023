use aoc_utils::PuzzleInput;
const DAY: u8 = 4;

#[derive(Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn parse(line: String) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let mut id_split = line.split(": ");
        let id = id_split
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let numbers_strings = id_split.next().unwrap().split(" | ");
        let winning_numbers_str = numbers_strings.clone().next().unwrap();
        let numbers_str = numbers_strings.clone().nth(1).unwrap();

        let winning_numbers = winning_numbers_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        let numbers = numbers_str
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    fn count_matching_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn calculate_points(&self) -> usize {
        let matching_numbers = self.count_matching_numbers();
        if matching_numbers == 0 {
            0
        } else {
            let base: usize = 2;
            base.pow(matching_numbers as u32 - 1)
        }
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    input
        .lines()
        .map(|line| Card::parse(line))
        .map(|card| card.calculate_points())
        .sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let available_cards: Vec<_> = input.lines().map(|line| Card::parse(line)).collect();
    let mut total_cards = available_cards.clone();
    let mut cards_left_to_process = available_cards.clone();

    while cards_left_to_process.len() > 0 {
        let card = cards_left_to_process.pop().unwrap();
        let match_count = card.count_matching_numbers();

        let new_cards = (card.id + 1..=card.id + match_count)
            .flat_map(|id| available_cards.iter().filter(|c| c.id == id).next());

        new_cards.for_each(|card| {
            total_cards.push(card.clone());
            cards_left_to_process.push(card.clone());
        })
    }

    total_cards.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn parses_correctly() {
        let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
    }

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        // takes about 6 GB RAM because it is not optimized
        // solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 13);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 30);
    }
}
