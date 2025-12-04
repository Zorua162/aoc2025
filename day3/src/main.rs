use std::fs;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: usize,
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
        let digit = char
            .to_digit(10)
            .expect("Expected a digit here, got {char}");
        if digit > highest {
            highest = digit;
            highest_loc = i;
        }
    }

    (
        highest.try_into().expect("Couldn't convert to usize"),
        highest_loc,
    )
}

fn exactly_two_highest(line: &str) -> usize {
    let (mut highest, highest_location) = find_highest_and_pos(line);

    let second_highest;

    if highest_location == line.len() - 1 {
        second_highest = highest;
        (highest, _) = find_highest_and_pos(&line[..line.len() - 1]);
    } else {
        // Run find_highest_and_pos on a slice from the given highest location
        (second_highest, _) = find_highest_and_pos(&line[(highest_location + 1)..]);
    }

    let value = format!("{highest}{second_highest}");
    println!("{value}");
    return value.parse().expect("Expected a value here");
}

fn part1(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");

    let mut answer: usize = 0;

    for line in contents.lines() {
        answer += exactly_two_highest(line)
    }

    Some(Answer { answer })
}

fn _twelve_highest_first_attempt(line: &str) -> usize {
    let mut out_value: Vec<usize> = vec![];

    let mut line_string = line.to_string();

    // if its an end value then add to the end, otherwise add to the start and remove from line!

    // while there isn't 12 values look for another

    let mut dist_from_front_loc = 0;
    let mut dist_from_back_loc = 0;

    while out_value.len() < 12 {
        let (highest, highest_location) = find_highest_and_pos(&line_string);

        if highest_location == line_string.len() - 1 {
            if dist_from_back_loc == 0 {
                out_value.push(highest);
            } else {
                let index = out_value.len() - dist_from_back_loc;
                println!("out_value {out_value:?} index {index} line {line_string}");

                out_value.insert(index, highest);
            }
            dist_from_back_loc += 1;
        } else {
            out_value.insert(dist_from_front_loc, highest);
            dist_from_front_loc += 1;
        }
        line_string.remove(highest_location);
    }

    let answer = out_value
        .iter()
        .map(|n| format!("{n}"))
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("Expected an integer");
    println!("{answer}");
    return answer;
}

fn _twelve_highest_weights(line: &str) -> usize {
    // Each is numbered 1 to 15
    // First is numbered 15, last 1
    // Subtract value from 10 to get its weight
    // Times weight by position
    // Remove highest weight in order

    let line_numbers: Vec<u32> = line
        .chars()
        .map(|n| n.to_digit(10).expect("Expected a digit here"))
        .collect();

    let weights: Vec<u32> = line_numbers
        .iter()
        .enumerate()
        // .map(|(i, n)| 15 - i.try_into().unwrap() * (10 - n).try_into().expect("Expected i32"))
        .map(|(i, n)| (15 - i as u32) * (10 - n))
        .collect();

    println!("weights {weights:?}");
    0
}

fn twelve_highest_scanner(line: &str) -> Result<usize, &'static str> {

    // Scan for the next highest number 9..0
    // (Save indexes)

    // i.e 234234234234278
    // Scan 9-1
    // For each value scan left to right, and switch them "on"
    // Cut off everything before the last largest number that was found

    let mut on_locations: Vec<bool> = vec![false; line.len()];

    let mut largest_number_loc: i32 = 0;
    let mut next_largest_number_loc: i32 = 0;

    let line_numbers: Vec<u32> = line
        .chars()
        .map(|n| n.to_digit(10).expect("Expected a digit here"))
        .collect();

    for i in (1..10).rev() {
        println!("trying {i} indexes {on_locations:?}");
        // let mut pointer: usize = line_numbers.len().try_into().expect("Expected i32 here");
        let mut pointer = line_numbers.len();

        // TODO: In the example line 3 for some reason the pointer "largest_number_loc" isn't blocking that extra 3 from being put in!!
        while i32::try_from(pointer).expect("i32") > largest_number_loc {
            println!("pointer {pointer} largest_number_loc {largest_number_loc}");
            pointer -= 1;
            if line_numbers[pointer] == i {
                on_locations[pointer] = true;
                next_largest_number_loc = pointer as i32;
            }
            if on_location_count(&on_locations) >= 12 {
                return Ok(on_locations_to_value(on_locations, line_numbers))
            }
        }
        largest_number_loc = next_largest_number_loc;

    }

    Err("All 12 values were not found")
}

fn on_locations_to_value(on_locations: Vec<bool>, line_numbers: Vec<u32>) -> usize {
    let mut out_value = vec![];

    for (i, val) in on_locations.iter().enumerate() {
        if *val {
            out_value.push(line_numbers[i])
        }
    }

    let answer = out_value
        .iter()
        .map(|n| format!("{n}"))
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("Expected an integer");
    answer
}

fn on_location_count(on_locations: &Vec<bool>) -> i32 {
    let mut count = 0;
    for i in on_locations {
        if *i {
            count += 1;
        }
    }
    count
}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");
    println!("---");


    let mut answer: usize = 0;

    for line in contents.lines() {
        let out = twelve_highest_scanner(line);
        if out.is_err() {
            panic!("Out value from line raised an error");
        }
        let value = out.expect("Value here");
        println!("value is {value}");
        answer += value;
    }

    Some(Answer { answer })
}

// Attempted answers

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let doing_part2 = true;
    if doing_part2 {
        let result2 = part2(&contents);
        println!("Part2 result {result2:?}");
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 17158 }));
    }

    #[test]
    fn test_part1_example() {
        let contents = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111"
            .to_string();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 357 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }

    #[test]
    fn test_part2_example() {
        let contents = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111"
            .to_string();
        let result = part2(&contents);
        assert_eq!(
            result,
            Some(Answer {
                answer: 3121910778619
            })
        );
    }
}
