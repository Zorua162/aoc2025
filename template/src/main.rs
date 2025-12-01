
use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: usize
}

trait InputGetter {
    fn get_input(&self) -> String;
    
}

struct LocalFileInputGetter {
    path: &'static str,
}

impl InputGetter for LocalFileInputGetter {
    fn get_input(&self) -> String {
        return fs::read_to_string(self.path).expect("Input file is expected");
    }
}

fn part1(contents: &String) -> Option<Answer> {
    None

}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    None

}

// Attempted answers

fn main() {

    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let result2 = part2(&contents);
    println!("Part2 result {result2:?}");

}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 2081}));
    }

    #[test]
    fn test_part2() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341}));
    }
}