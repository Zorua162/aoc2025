use std::fs;

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
    println!("Contents is {contents}");
    None

}

// Part 1 attempted answers

fn part2(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");
    None

}

// Part 2 attempted answers

fn main() {

    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let doing_part1 = true;
    if doing_part1 {
        let result1 = part1(&contents);
        println!("Part1 result {result1:?}");
    } else {
        let result2 = part2(&contents);
        println!("Part2 result {result2:?}");
    }
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        contents: String,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                contents: "..@@.@@@@.\n\
                            @@@.@.@.@@\n\
                            @@@@@.@.@@\n\
                            @.@@@@..@.\n\
                            @@.@@@@.@@\n\
                            .@@@@@@@.@\n\
                            .@.@.@.@@@\n\
                            @.@@@.@@@@\n\
                            .@@@@@@@@.\n\
                            @.@.@@@.@."
                .to_string(), 
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 13 }));
    }

    #[ignore]
    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 1395 }));
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 43 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}