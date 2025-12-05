use std::fs;

#[derive(Debug, PartialEq)]
struct Answer {
    answer: u64,
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

#[derive(Debug, PartialEq)]
struct IDRange {
    start: u64,
    end: u64,
}

impl IDRange {
    fn check_in_range(&self, id: u64) -> bool {
        return id >= self.start && id <= self.end;
    }
}

fn parse_data(contents: &String) -> (Vec<IDRange>, Vec<u64>) {
    let data: Vec<&str> = contents.split("\n\n").collect();
    dbg!(&data);

    let fresh_ingredients_id_ranges = data[0];
    let ingredients = data[1];

    let mut fresh_ingredients_ranges: Vec<IDRange> = vec![];

    for line in fresh_ingredients_id_ranges.lines() {
        let split_line: Vec<&str> = line.split("-").collect();
        let start: u64 = split_line[0]
            .parse()
            .expect("Expected an interger here for start");

        let end = split_line[1]
            .parse::<u64>()
            .expect("Expected an interger here for start");
        fresh_ingredients_ranges.push(IDRange { start, end });
    }

    let available_ingredients: Vec<u64> = ingredients
        .lines()
        .map(|line| line.parse::<u64>().expect("Expected an integer here"))
        .collect();

    return (fresh_ingredients_ranges, available_ingredients);
}

fn part1(contents: &String) -> Option<Answer> {
    // println!("Contents is {contents}");
    let mut answer = 0;
    let (fresh_ingredients_ranges, available_ingredients) = parse_data(contents);

    for ingredient_id in available_ingredients {
        for range in &fresh_ingredients_ranges {
            if range.check_in_range(ingredient_id) {
                println!("id {ingredient_id} is fresh");
                answer += 1;
                break;
            }
        }
    }

    return Some(Answer { answer });
}

// Part 1 attempted answers

fn part2(contents: &String) -> Option<Answer> {
    let mut answer = 0;
    println!("Contents is {contents}");

    let (fresh_ingredients_ranges, _) = parse_data(contents);

    let overlaps_removed_fresh_ingredients_ranges = deoverlap_id_ranges(fresh_ingredients_ranges);

    for range in overlaps_removed_fresh_ingredients_ranges {
        println!("Range {range:?}");
        answer += range.end - range.start + 1;
    }

    return Some(Answer { answer });
}

fn deoverlap_id_ranges(mut fresh_ingredients_ranges: Vec<IDRange>) -> Vec<IDRange> {
    let mut out_ranges: Vec<IDRange> = vec![];

    fresh_ingredients_ranges.sort_by_key(|r| r.start);

    println!("{fresh_ingredients_ranges:?}");

    let mut pointer = 0;

    for range in &fresh_ingredients_ranges {
        let mut out_start = range.start;

        if pointer >= range.end {
            continue;
        }

        if range.check_in_range(pointer) {
            out_start = pointer + 1;
        }

        pointer = range.end;

        let new_range = IDRange {
            start: out_start,
            end: range.end,
        };
        // if new_range.start < new_range.end {
        //     out_ranges.push(new_range);
        // }
        out_ranges.push(new_range);
    }
    return out_ranges;
}

// Part 2 attempted answers
// 372428504353007 too high
// 350939902751909

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let doing_part1 = false;
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
                contents: "3-5\n\
                           10-14\n\
                           16-20\n\
                           12-18\n\n\
                           1\n\
                           5\n\
                           8\n\
                           11\n\
                           17\n\
                           32"
                .to_string(),
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 3 }));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 511 }));
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 14 }));
    }

    #[test]
    fn test_part2_self_made_data() {
        let contents = "1-3\n\
                                4-6\n\
                                3-9\n\
                                8-11\n\n\
                                1\n\
                                2"
        .to_string();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 11 }));
    }

    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 350939902751909 }));
    }
}
