use std::{cmp::Ordering, collections::HashMap};

use aoc_utils::PuzzleInput;
const DAY: u8 = 7;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("Invalid card: {}", c),
        }
    }

    fn all_cards() -> Vec<Self> {
        vec![
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::T,
            Self::J,
            Self::Q,
            Self::K,
            Self::A,
        ]
    }

    fn card_to_num(&self, jokers_enabled: bool) -> i8 {
        if jokers_enabled && *self == Self::J {
            -1
        } else {
            *self as i8
        }
    }

    fn cmp_card(&self, other: &Self, jokers_enabled: bool) -> Ordering {
        self.card_to_num(jokers_enabled)
            .cmp(&other.card_to_num(jokers_enabled))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct CardHand {
    cards: [Card; 5],
    bid: u16,
}

impl CardHand {
    fn parse(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let cards: Vec<_> = split.next().unwrap().chars().map(Card::new).collect();
        let bid = split.next().unwrap().parse::<u16>().unwrap();

        Self {
            cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
            bid,
        }
    }

    fn get_type(&self, jokers_enabled: bool) -> CardHandType {
        let all_possible_cards = Card::all_cards();
        let mut group_by_count = HashMap::new();

        for card in self.cards.iter() {
            if jokers_enabled && *card == Card::J {
                for card in all_possible_cards.iter() {
                    let count = group_by_count.entry(card).or_insert(0);
                    *count += 1;
                }
            } else {
                let count = group_by_count.entry(card).or_insert(0);
                *count += 1;
            }
        }

        // Five of a kind
        if group_by_count.values().any(|&v| v >= 5) {
            return CardHandType::FiveOfAKind;
        }

        // Four of a kind
        if group_by_count.values().any(|&v| v == 4) {
            return CardHandType::FourOfAKind;
        }

        // Full house
        if group_by_count.values().any(|&v| v >= 3) && group_by_count.values().any(|&v| v >= 2) {
            return CardHandType::FullHouse;
        }

        // Three of a kind
        if group_by_count.values().any(|&v| v >= 3) {
            return CardHandType::ThreeOfAKind;
        }

        // Two pair
        if group_by_count.values().filter(|&&v| v >= 2).count() == 2 {
            return CardHandType::TwoPair;
        }

        // One pair
        if group_by_count.values().any(|&v| v >= 2) {
            return CardHandType::OnePair;
        }

        // High card
        CardHandType::HighCard
    }

    fn cmp_hand(&self, other: &Self, jokers_enabled: bool) -> Ordering {
        let self_type = self.get_type(jokers_enabled);
        let other_type = other.get_type(jokers_enabled);

        if self_type == other_type {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].cmp_card(&other.cards[i], jokers_enabled);
                }
            }
            Ordering::Equal
        } else {
            self_type.cmp(&other_type)
        }
    }
}

fn calculate_score(input: &PuzzleInput, jokers_enabled: bool) -> usize {
    let mut hands = input
        .lines()
        .map(|line| CardHand::parse(&line))
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| a.cmp_hand(b, jokers_enabled));
    // hands
    //     .iter()
    //     .for_each(|hand| println!("{:?} {:?}", hand, hand.get_type(jokers_enabled)));

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid as usize * (i + 1))
        .sum()
}

fn solve_a(input: &PuzzleInput) -> usize {
    calculate_score(input, false)
}

fn solve_b(input: &PuzzleInput) -> usize {
    calculate_score(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 6440);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 5905);
    }
}
