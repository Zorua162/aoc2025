use std::{collections::VecDeque, fs};


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

fn twelve_highest_slide(line: &str) -> Result<usize, &'static str> {
    // We start with the first 12 values from the line
    // we then iterate down the rest of the line, if the value is larger than the
    // number at the end it "slides" onto the end, this is done all the way down the line
    // from the end of it back to the start
    // Eventually there will be only 12 values after that can be "slid" to

    // For example
    // Our input line is
    // 234234234234278

    // We start with the first 12 values
    // 234234234234

    // We start a pointer at 0

    // the pointer looks at the second value
    // if the next value before is smaller , and there are remaining values to parse available
    // then the prior-smaller number gets deleted
    // If this number is smaller, then the pointer just goes to the next number

    // We still need to think about the situation where the pointer can be moved back

    // Once the "processed" list is empty, then we have our highest number!

    let values: Vec<u32> = line
        .chars()
        .map(|n| {
            n.to_digit(10)
                .expect(&format!("Expected a digit here, found {n}"))
        })
        .collect();

    let mut current_answer: Vec<u32> = values[..12].to_vec();

    let mut to_parse: VecDeque<u32> = values[12..].to_vec().into();

    let mut pointer = 0;

    println!("line is {line}");

    while to_parse.len() > 0 {
        pointer += 1;

        println!("pointer {pointer} current_answer {current_answer:?} to_parse {to_parse:?}");
        // If pointer is 12 then look at the first value in to_parse
        // If the pointer is currently within the answer
        if pointer >= 12 {
            let next_value = to_parse[0];
            to_parse.remove(0);

            if next_value > current_answer[11] {
                current_answer.remove(11);
                current_answer.push(next_value);
                // Situation could be like
                // 555555555555 655555
                // In which case that 6 now needs to be moved
                // into the location of all those 5s
            }
            pointer = 0;
        } else {
            // Len check probably unecessary
            if current_answer[pointer] > current_answer[pointer - 1] && to_parse.len() > 0 {
                current_answer.remove(pointer - 1);
                current_answer.push(to_parse.pop_front().expect(&format!("Expected a value in to_parse {to_parse:?}")));
                pointer = 0;
            }
        }
    }

    let answer = current_answer
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect(&format!(
            "Expected answer to be a number is is instead {current_answer:?}"
        ));

    return Ok(answer);
}

// ttempted answers

fn part2(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");
    println!("---");

    let mut answer: usize = 0;

    for line in contents.lines() {
        let out = twelve_highest_slide(line);
        if out.is_err() {
            panic!("Out value from line raised an error");
        }
        let value = out.expect("Value here");
        println!("---\nline {line} \nvalue is {value}");
        answer += value;
    }

    Some(Answer { answer })
}

// Attempted answers
// 150209522862244 too low
// 169861939309812 too low
// 92911667104343 too low
// 166861249550998
// 170449335646486

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

    #[test]
    fn test_part2_breaking_1() {
        let contents = "4433233445334333433332243332243333323333244341233329233322351213324333213433242123334332332622363223".to_string();
        let result = part2(&contents);
        let value = result.expect("Expecting an Answer").answer;
        dbg!(value);
        assert_eq!(value.to_string().len(), 12);
        // assert_eq!(
        //     result,
        //     Some(Answer {
        //         answer: 3121910778619
        //     })
        // );
    }

    #[test]
    fn test_part2_incorrect_1() {
        let contents = "2222222123222222232282222725322229122222222223312132222222712122322222222213121322522227222222243323".to_string();
        let result = part2(&contents);
        let value = result.expect("Expecting an Answer").answer;
        assert_eq!(value, 977222243323);
    }

    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 170449335646486 }));
    }
}
