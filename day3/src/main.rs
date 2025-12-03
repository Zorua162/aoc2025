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

fn find_highest_and_pos(line: &str) -> (usize, usize) {

    let mut highest = 0;
    let mut highest_loc = 0;

    for (i, char) in line.chars().enumerate() {

        let digit = char.to_digit(10).expect("Expected a digit here, got {char}");
        if digit > highest {
            highest = digit;
            highest_loc = i;
        }
    }

    (highest.try_into().expect("Couldn't convert to usize"), highest_loc)
}

fn exactly_two_highest(line: &str) -> usize {

    let (highest, highest_location) = find_highest_and_pos(line);
    
    // Run find_highest_and_pos on a slice from the given highest location
    let (second_highest, _) = find_highest_and_pos(&line[(highest_location+1)..]);

    println!("{highest} x {second_highest} ");

    highest * second_highest
}

fn part1(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");

    let mut answer: usize = 0;

    for line in contents.lines() {
        answer += exactly_two_highest(line)
    }

    Some(Answer { answer })

}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");
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


    #[ignore]
    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 2081}));
    }

    #[test]
    fn test_part1_example() {
        let contents = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111".to_string();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 357}));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341}));
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        let contents = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111".to_string();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 357}));
    }
}