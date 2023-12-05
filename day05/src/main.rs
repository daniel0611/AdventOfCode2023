use std::ops::RangeInclusive;

use aoc_utils::PuzzleInput;
const DAY: u8 = 5;
const START_CATEGORY: &str = "seed";
const END_CATEGORY: &str = "location";

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn range_split_at(
    range: RangeInclusive<usize>,
    split_at: usize,
) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let left = (*range.start())..=split_at;
    let right = (split_at + 1)..=(*range.end());

    (left, right)
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

    fn map_range(&self, num_range: &RangeInclusive<usize>) -> Vec<RangeInclusive<usize>> {
        let range_overlap = Self::get_overlapping_range(
            num_range,
            &(self.source_start..=self.source_start + self.range_length),
        );
        if range_overlap.is_none() {
            // no overlap, return the original range and do nothing
            return vec![num_range.clone()];
        }
        let range_overlap = range_overlap.unwrap();
        println!("Overlap: {:?}", range_overlap);
        println!("Range: {:?}", num_range);

        // Determine whether the range is fully contained in the translation range
        // or whether it is partially contained and we need to split num_range
        if range_overlap == *num_range {
            // The range is fully contained in the translation range
            // so we can just translate it
            let start = *num_range.start();
            let offset = start - self.source_start;
            let destination_element_start = self.destination_start + offset;

            let range_length = *num_range.end() - *num_range.start();
            vec![destination_element_start..=destination_element_start + range_length]
        } else {
            // The range is partially contained in the translation range
            // so we need to split it at overlap end or start depending on position
            if *num_range.start() < *range_overlap.start() {
                // Split at overlap start
                let (left, right) = range_split_at(num_range.clone(), *range_overlap.start());
                println!(
                    "Range: {:?}, Left: {:?}, Right: {:?}",
                    num_range, left, right
                );
                self.map_range(&left)
                    .into_iter()
                    .chain(self.map_range(&right))
                    .collect()
            } else {
                // Split at overlap end
                let (left, right) = range_split_at(num_range.clone(), *range_overlap.end());
                self.map_range(&left)
                    .into_iter()
                    .chain(self.map_range(&right))
                    .collect()
            }
        }
    }

    fn get_overlapping_range(
        a: &RangeInclusive<usize>,
        b: &RangeInclusive<usize>,
    ) -> Option<RangeInclusive<usize>> {
        let a_range = a.start()..=a.end();
        let b_range = b.start()..=b.end();

        if a_range.start() <= b_range.end() && b_range.start() <= a_range.end() {
            let start = a_range.start().max(b_range.start());
            let end = a_range.end().min(b_range.end());

            Some(**start..=**end)
        } else {
            None
        }
    }

    fn is_in_range(&self, num: &RangeInclusive<usize>) -> bool {
        let r = self.source_start..=self.source_start + self.range_length;
        // println!(
        //     "{} {}\t{} {}\t{}",
        //     *r.start(),
        //     *r.end(),
        //     *num.start(),
        //     *num.end(),
        //     *r.start() <= *num.end() && *num.start() <= *r.end()
        // );

        // Check whether the range overlaps with the translation range
        // *r.start() <= *num.end() && *num.start() <= *r.end()
        Self::get_overlapping_range(&r, num).is_some()
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
    initial_seeds: Vec<RangeInclusive<usize>>,
}

impl Almanac {
    fn parse(input: &PuzzleInput, seeds_are_ranges: bool) -> Self {
        let initial_seeds = input
            .lines()
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(' ')
            .flat_map(|str| str.parse())
            .collect::<Vec<usize>>();

        let initial_seed_ranges: Vec<_> = if seeds_are_ranges {
            initial_seeds
                .chunks(2)
                .map(|seed_range| {
                    let start = seed_range[0];
                    let length = seed_range[1];

                    start..=start + length
                })
                .collect()
        } else {
            initial_seeds.iter().map(|num| *num..=*num).collect()
        };

        let input_without_initial_seeds = input.lines().skip(2).collect::<Vec<_>>().join("\n");
        let translation_maps = input_without_initial_seeds
            .split("\n\n")
            .map(TranslationMap::parse)
            .collect();

        Self {
            translation_maps,
            initial_seeds: initial_seed_ranges,
        }
    }

    fn translate_till_end_location(&self) -> Vec<RangeInclusive<usize>> {
        let mut current_category = START_CATEGORY;
        let mut number_ranges = self.initial_seeds.clone();
        println!("Number ranges: {:?}", number_ranges);

        while current_category != END_CATEGORY {
            let map = self
                .translation_maps
                .iter()
                .find(|map| map.source_type == current_category)
                .unwrap();

            number_ranges = number_ranges
                .iter()
                .flat_map(|number_range| {
                    let mapping = map
                        .ranges
                        .iter()
                        .find(|translation_map| translation_map.is_in_range(number_range));
                    match mapping {
                        Some(mapping) => mapping.map_range(number_range),
                        None => vec![number_range.clone()],
                    }
                })
                .collect();
            current_category = &map.destination_type;

            println!(
                "Translated from {} to {}",
                map.source_type, map.destination_type
            );
            println!("Number ranges: {:?}", number_ranges);
        }

        number_ranges
    }

    fn get_minimum_end_category_number(&self) -> usize {
        self.translate_till_end_location()
            .iter()
            .map(|range| *range.start())
            .min()
            .unwrap()
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    Almanac::parse(input, false).get_minimum_end_category_number()
}

fn solve_b(input: &PuzzleInput) -> usize {
    Almanac::parse(input, true).get_minimum_end_category_number()
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
        // solve_b(&input);
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
