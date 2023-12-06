use aoc_utils::PuzzleInput;
const DAY: u8 = 6;

struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn new(time: usize, record_distance: usize) -> Self {
        Self {
            time,
            record_distance,
        }
    }

    fn does_beat_record(&self, hold_time: usize) -> bool {
        let speed = hold_time;
        let distance = speed * (self.time - hold_time);
        distance > self.record_distance
    }

    fn determine_record_beating_hold_times(&self) -> Vec<usize> {
        (0..=self.time)
            .filter(|hold_time| self.does_beat_record(*hold_time))
            .collect()
    }
}

fn parse_races(input: &PuzzleInput) -> Vec<Race> {
    let time_line = input.lines().next().unwrap();
    let distance_line = input.lines().nth(1).unwrap();

    let times = time_line.split_whitespace().flat_map(|s| s.parse());
    let distances = distance_line.split_whitespace().flat_map(|s| s.parse());

    times
        .zip(distances)
        .map(|(time, distance)| Race::new(time, distance))
        .collect()
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> usize {
    let races = parse_races(input);

    races
        .iter()
        .map(|g| g.determine_record_beating_hold_times().len())
        .product()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let races = parse_races(input);
    let big_race = races.iter().fold(Race::new(0, 0), |a, b| {
        let time_str = a.time.to_string() + &b.time.to_string();
        let distance_str = a.record_distance.to_string() + &b.record_distance.to_string();
        Race::new(time_str.parse().unwrap(), distance_str.parse().unwrap())
    });

    big_race.determine_record_beating_hold_times().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 288);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 71503);
    }
}
