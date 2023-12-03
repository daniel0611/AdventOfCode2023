use aoc_utils::PuzzleInput;
const DAY: u8 = 2;

#[derive(Copy, Clone, PartialEq, Eq)]
enum CubeColor {
    Blue,
    Red,
    Green,
}

impl CubeColor {
    fn parse(s: &str) -> Self {
        match s {
            "blue" => Self::Blue,
            "red" => Self::Red,
            "green" => Self::Green,
            _ => panic!("Invalid color"),
        }
    }
}

struct CubeSet {
    cubes: Vec<CubeColor>,
}

struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(": ");
        let id = parts
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let mut cube_sets = Vec::new();
        for cube_set in parts.next().unwrap().split("; ") {
            let mut cubes = Vec::new();
            for cube in cube_set.split(", ") {
                let mut parts = cube.split(' ');
                let count = parts.next().unwrap().parse().unwrap();
                let color = CubeColor::parse(parts.next().unwrap());
                for _ in 0..count {
                    cubes.push(color);
                }
            }
            cube_sets.push(CubeSet { cubes });
        }

        Self { id, cube_sets }
    }

    fn minimum_needed_cubes(&self, color: CubeColor) -> usize {
        self.cube_sets
            .iter()
            .map(|cube_set| cube_set.cubes.iter().filter(|c| **c == color).count())
            .max()
            .unwrap()
    }

    fn can_be_played(&self) -> bool {
        self.minimum_needed_cubes(CubeColor::Blue) <= 14
            && self.minimum_needed_cubes(CubeColor::Green) <= 13
            && self.minimum_needed_cubes(CubeColor::Red) <= 12
    }

    fn calculate_power(&self) -> usize {
        self.minimum_needed_cubes(CubeColor::Blue)
            * self.minimum_needed_cubes(CubeColor::Green)
            * self.minimum_needed_cubes(CubeColor::Red)
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
        .map(|line| Game::parse(&line))
        .filter(|game| game.can_be_played())
        .map(|game| game.id)
        .sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    input
        .lines()
        .map(|line| Game::parse(&line))
        .map(|game| game.calculate_power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 8);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 2286);
    }
}
