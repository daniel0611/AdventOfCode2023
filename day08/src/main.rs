use std::collections::HashMap;

use aoc_utils::PuzzleInput;
const DAY: u8 = 8;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Option<Self> {
        match c {
            'L' => Some(Self::Left),
            'R' => Some(Self::Right),
            _ => None,
        }
    }
}

type RoadPosition = [char; 3];

fn parse_road_position(s: &str) -> Option<RoadPosition> {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        .take(3)
        .collect::<Vec<_>>()
        .try_into()
        .ok()
}

struct RoadFork {
    left: RoadPosition,
    right: RoadPosition,
}

impl RoadFork {
    fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split(',');
        let left = parse_road_position(parts.next().unwrap().trim())?;
        let right = parse_road_position(parts.next().unwrap().trim())?;
        Some(Self { left, right })
    }
}

struct Map {
    directions: Vec<Direction>,
    forkings: HashMap<RoadPosition, RoadFork>,
}

impl Map {
    fn parse(input: &PuzzleInput) -> Self {
        let mut lines = input.lines();
        let directions = lines
            .next()
            .unwrap()
            .chars()
            .filter_map(Direction::parse)
            .collect::<Vec<_>>();

        let forkings = lines
            .skip(1) // empty line between directions and forkings
            .map(|line| {
                let mut parts = line.split('=');
                let source = parse_road_position(parts.next().unwrap().trim()).unwrap();
                let fork = RoadFork::parse(parts.next().unwrap().trim()).unwrap();
                (source, fork)
            })
            .collect();

        Self {
            directions,
            forkings,
        }
    }

    fn part_a(&self) -> usize {
        let mut position = parse_road_position("AAA").unwrap();
        let end_position = parse_road_position("ZZZ").unwrap();
        let mut steps = 0;

        while position != end_position {
            let direction = &self.directions[steps % self.directions.len()];
            let fork = self.forkings.get(&position).unwrap();
            position = match direction {
                Direction::Left => fork.left,
                Direction::Right => fork.right,
            };
            steps += 1;
        }

        steps
    }

    fn part_b(&self) -> usize {
        let starting_positions = self.forkings.keys().cloned().filter(|p| p[2] == 'A');

        // Get the steps for each position to be back at the start (cycle length)
        let cycle_lengths = starting_positions.map(|p| {
            let mut position = p;
            let mut steps = 0;
            while position[2] != 'Z' {
                let direction = &self.directions[steps % self.directions.len()];
                let fork = self.forkings.get(&position).unwrap();
                position = match direction {
                    Direction::Left => fork.left,
                    Direction::Right => fork.right,
                };
                steps += 1;
            }
            steps
        });

        // Step count where all positions are at a valid end position is when the lowest common
        // multiple of all cycle lengths is reached
        cycle_lengths.fold(1, Self::lcm)
    }

    fn lcm(a: usize, b: usize) -> usize {
        a * b / Self::gcd(a, b)
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    Map::parse(input).part_a()
}

fn solve_b(input: &PuzzleInput) -> usize {
    Map::parse(input).part_b()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_A: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_B: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT_A)), 2);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT_B)), 6);
    }
}
