
use std::fs;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: i32
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

fn do_rotation(current_loc: i32, direction: char, number: i32) -> i32{
    let mut current_loc: i32 = current_loc;

    if direction == 'R' {
        current_loc += number;

        while current_loc > 99 {
            current_loc -= 100;
        }
    } else {
        current_loc -= number;

        while current_loc < 0 {
            current_loc += 100;
        }

    }

    current_loc
}

fn do_rotation_part2(current_loc: i32, zero_count: i32, direction: char, number: i32) -> (i32, i32){
    let mut current_loc: i32 = current_loc;
    let mut zero_count: i32 = zero_count;

    let change;

    if direction == 'R' {
        change = 1;
    } else {
        change = -1;
    }

    for _ in 0..number {
        current_loc += change;

        match current_loc {
            100 => {
                current_loc = 0;
            }
            -1 => current_loc = 99,
            _ => (),
        }
        
        if current_loc == 0 {
            println!("Added one to the zero count");
            zero_count += 1;
        }

    }

    (current_loc, zero_count)
}

fn part1(contents: &String) -> Option<Answer> {

    let mut current_loc = 50;

    let mut zero_count = 0;

    for line in contents.lines() {
        println!("Line is {line}");
        let direction = line.to_string().chars().nth(0).expect("Expected first letter");
        let number = line.to_string()[1..].parse().expect("Expected a number");
        println!("{current_loc}, {direction}, {number}");
        current_loc = do_rotation(current_loc, direction, number);
        if current_loc == 0 {
            zero_count += 1;
        }
    }
    Some(Answer{ answer: zero_count})
}

// Attempted answers
// 287, too low

fn part2(contents: &String) -> Option<Answer> {
    let mut current_loc = 50;

    let mut zero_count: i32 = 0;

    for line in contents.lines() {
        println!("----");
        println!("Line is {line}");
        let direction = line.to_string().chars().nth(0).expect("Expected first letter");
        let number = line.to_string()[1..].parse().expect("Expected a number");
        println!("{current_loc}, {direction}, {number}");
        (current_loc, zero_count) = do_rotation_part2(current_loc, zero_count, direction, number);
        // if current_loc == 0 {
        //     zero_count += 1;
        //     println!("Ended up at 0, so adding 1");
        // }
    }
    Some(Answer{ answer: zero_count})

}

// Attempted answers
// 8411, too high
// 7243, too high

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
    fn test_do_rotation() {
        let result1 = do_rotation(11, 'R', 8);
        assert_eq!(19, result1);

        let result2 = do_rotation(19, 'L', 19);
        assert_eq!(0, result2);
    }

    #[test]
    fn test_rotate_boundary() {
        let result1 = do_rotation(99, 'R', 1);
        assert_eq!(0, result1);

        let result2 = do_rotation(0, 'L', 1);
        assert_eq!(99, result2);
    }

    #[test]
    fn test_rotate_more_than_200() {
        let result1 = do_rotation(50, 'R', 210);
        assert_eq!(60, result1);

        let result2 = do_rotation(50, 'L', 210);
        assert_eq!(40, result2);
    }

    #[test]
    fn test_integration() {
        let contents = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82".to_string();

        let result = part1(&contents);
        
        assert_eq!(3, result.expect("Value").answer);
    }

    #[test]
    fn test_part1() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 1168}));
    }

    #[test]
    fn test_integration_part2() {
        let contents = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82".to_string();

        let result = part2(&contents);
        
        assert_eq!(6, result.expect("Value").answer);
    }

    #[test]
    fn test_rotate_1000_part2() {
        let (location, zero_count) = do_rotation_part2(50,0, 'R', 1000);
        assert_eq!(location, 50);
        assert_eq!(zero_count, 10);

    }

    #[ignore]
    #[test]
    fn test_part2() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341}));
    }
}