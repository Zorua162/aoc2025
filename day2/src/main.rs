use std::{fs, string};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: i128
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

fn is_invalid_part1(value: i128) -> bool{
    // True means it is invalid
    // println!("{value}");

    let string_value = value.to_string();

    // Sequence of digits repeated twice

    let left = &string_value[0..string_value.len()/2];
    let right= &string_value[string_value.len()/2..];

    // println!("left {left} right {right}");

    if left == right {
        println!("{value} is invalid");
        return true
    }


    // No leading zeros?

    false
}

fn is_invalid_part2(value: i128) -> bool {
    // True means it is invalid
    // println!("Trying value {value}");

    let string_value = value.to_string();

    // Sequence of digits repeated twice

    // let left = &string_value[0..string_value.len()/2];
    // let right= &string_value[string_value.len()/2..];

    // // println!("left {left} right {right}");

    // if left == right {
    //     println!("{value} is invalid");
    //     return true
    // }

    // 1 len-2
    // look at first 3 and see if they are repeated

    for i in 1..(string_value.len()/2+1) {
        let check_string = &string_value[0..i];
        if string_value.len() % i != 0 {
            continue
        }
        if matches_check_string(&string_value, i, check_string) {
            return true
        }


    }
    false
}

fn matches_check_string(string_value: &String, i: usize, check_string: &str) -> bool {
    let num_iterations = string_value.len()/i;
    // println!("iterations are {num_iterations} i is {i}");
    for j in 0..num_iterations {
        let match_value = &string_value[j*i..(j+1)*i];
        // println!("check_string {check_string} == match {match_value}");
        if check_string != match_value {
            return false
        }
    }
    return true

}

fn part1(contents: &String) -> Option<Answer> {
    let lines = contents.split(",");
    let mut invalid_count: i128 = 0;
    for line in lines {
        let values = line.split("-").collect::<Vec<&str>>();
        println!("{line}");
        let start: i128 = values.get(0).expect("Expected first value here").parse().expect("Expected value");
        let end: i128 = values.get(1).expect("Expected second value here").parse().expect("Expected value");
        println!("{start} {end}");

        for i in start..end+1 {
            if is_invalid_part1(i) {
                invalid_count += i;
            }
        }
    }

    Some(Answer{ answer: invalid_count})
}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    let lines = contents.split(",");
    let mut invalid_count: i128 = 0;
    for line in lines {
        let values = line.split("-").collect::<Vec<&str>>();
        // println!("{line}");
        let start: i128 = values.get(0).expect("Expected first value here").parse().expect("Expected value");
        let end: i128 = values.get(1).expect("Expected second value here").parse().expect("Expected value");
        // println!("start {start} end {end}");

        for i in start..end+1 {
            if is_invalid_part2(i) {
                // println!("{i} is invalid");
                invalid_count += i;
            }
        }
    }

    Some(Answer{ answer: invalid_count})
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
    fn test_part1_example() {
    let contents = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 1227775554}));
    }

    #[test]
    fn test_part1() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 43952536386}));
    }

    #[test]
    fn test_part2_example() {
    let contents = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 4174379265}));
    }

    #[ignore]
    #[test]
    fn test_part2() {
    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341}));
    }
}