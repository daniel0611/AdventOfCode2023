use aoc_utils::PuzzleInput;
const DAY: u8 = 5;
const START_CATEGORY: &str = "seed";
const END_CATEGORY: &str = "location";

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

struct TranslationRange {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl TranslationRange {
    fn parse(line: &str) -> Self {
        let numbers = line
            .split(' ')
            .flat_map(|str| str.parse())
            .collect::<Vec<_>>();
        if numbers.len() != 3 {
            panic!("Invalid translation range: {}", line);
        }

        Self {
            destination_start: numbers[0],
            source_start: numbers[1],
            range_length: numbers[2],
        }
    }

    fn map_number(&self, num: usize) -> usize {
        if !self.is_in_range(num) {
            // out of range, don't translate
            num
        } else {
            let offset = num - self.source_start;
            self.destination_start + offset
        }
    }

    fn is_in_range(&self, num: usize) -> bool {
        num >= self.source_start && num < self.source_start + self.range_length
    }
}

struct TranslationMap {
    source_type: String,
    destination_type: String,
    ranges: Vec<TranslationRange>,
}

impl TranslationMap {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let map_name_line = lines.next().unwrap().replace(" map:", "");
        let types = map_name_line.split("-to-").collect::<Vec<_>>();
        let (source_type, destination_type) = (types[0].into(), types[1].into());

        let ranges = lines.map(TranslationRange::parse).collect();

        Self {
            source_type,
            destination_type,
            ranges,
        }
    }
}

struct Almanac {
    translation_maps: Vec<TranslationMap>,
    initial_seeds: Vec<usize>,
}

impl Almanac {
    fn parse(input: &PuzzleInput) -> Self {
        let initial_seeds = input
            .lines()
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" ")
            .flat_map(|str| str.parse())
            .collect();

        let input_without_initial_seeds = input.lines().skip(2).collect::<Vec<_>>().join("\n");
        let translation_maps = input_without_initial_seeds
            .split("\n\n")
            .map(TranslationMap::parse)
            .collect();

        Self {
            translation_maps,
            initial_seeds,
        }
    }

    fn translate_till_end_location(&self) -> Vec<usize> {
        let mut current_category = START_CATEGORY;
        let mut numbers = self.initial_seeds.clone();

        while current_category != END_CATEGORY {
            let map = self
                .translation_maps
                .iter()
                .find(|map| map.source_type == current_category)
                .unwrap();

            numbers = numbers
                .iter_mut()
                .map(|num| {
                    let translation_range = map.ranges.iter().find(|range| range.is_in_range(*num));

                    match translation_range {
                        Some(range) => range.map_number(*num),
                        None => *num,
                    }
                })
                .collect();
            current_category = &map.destination_type;
        }

        numbers
    }

    fn get_minimum_end_category_number(&self) -> usize {
        self.translate_till_end_location()
            .iter()
            .min()
            .unwrap()
            .clone()
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    Almanac::parse(input).get_minimum_end_category_number()
}

fn solve_b(input: &PuzzleInput) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 35);
    }

    // #[test]
    // fn test_solve_b() {
    //     assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 46);
    // }
}
