use std::fs;
use std::iter::Map;
use std::path::Path;

pub struct PuzzleInput {
    pub raw_input: String,
}

impl PuzzleInput {
    pub fn new<S: Into<String>>(content: S) -> PuzzleInput {
        PuzzleInput {
            raw_input: content.into(),
        }
    }

    /// Reads the puzzle input for the puzzle with the given day from input_day<day_number>.txt
    pub fn get_input(day: u8) -> PuzzleInput {
        let path_string = format!("input_day{:02}.txt", day);
        let path = Path::new(&path_string);

        // When running in tests, the working directory is inside the package, but
        // when running the actual puzzle, it's in the workspace root.
        let outside_path_str = format!("day{:02}/{}", day, path.display());
        let outside_path = Path::new(&outside_path_str);

        if path.exists() {
            let content = fs::read_to_string(path).expect("Unable to read file");
            PuzzleInput::new(content)
        } else if outside_path.exists() {
            let content = fs::read_to_string(outside_path).expect("Unable to read file");
            PuzzleInput::new(content)
        } else {
            let cwd = std::env::current_dir().unwrap();
            panic!(
                "Puzzle input at {}/{} does not exist",
                cwd.display(),
                path.display()
            );
        }
    }

    pub fn lines(&self) -> Map<std::str::Lines, fn(&str) -> String> {
        self.raw_input.lines().map(|s| s.to_string())
    }

    pub fn convert_to_ints_by_line<S: std::str::FromStr>(&self) -> Vec<S> {
        self.lines()
            .filter_map(|s| s.parse::<S>().ok())
            .collect()
    }

    pub fn convert_to_ints<S: std::str::FromStr>(&self) -> Vec<S> {
        self.raw_input
            .split(',')
            .map(|s| s.to_string())
            .filter_map(|s| s.parse::<S>().ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // Should be able to get input "Hello, this is a test" from day 0
    #[test]
    fn test_get_input_0_success() {
        let input = super::PuzzleInput::get_input(0);
        assert_eq!(input.raw_input, "Hello, this is a test\n");
    }

    // File for puzzle 1 doesn't exist in this directory, so should panic
    #[test]
    #[should_panic]
    fn test_get_input_1b_fail() {
        super::PuzzleInput::get_input(1);
    }
}
