use std::{fs, ptr::read_volatile};

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

struct Location {
    x: i32,
    y: i32,
}

fn check_loc_adjacent(location: Location, roll_data: &Vec<Vec<char>>) -> bool {
    // Search in 3x3 pattern around the roll to count how many rolls are there
    let mut roll_count = 0;

    // ...
    // .@.
    // ...

    let directions: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    for direction in directions {
        let xindex = location.x - direction.0;
        let yindex = location.y - direction.1;

        if let Some(line) = roll_data.get(xindex as usize) {
            if let Some(val) = line.get(yindex as usize) {
                if val == &'@' || val == &'x' {
                    roll_count += 1;
                }
            }
        }
    }

    return roll_count < 4;
}

fn part1(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");

    let mut answer = 0;

    let mut roll_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    if let Some(first_line) = contents.lines().next() {
        for (i, _) in contents.lines().enumerate() {
            for (j, _) in first_line.chars().enumerate() {
                if roll_data[i][j] == '@' && check_loc_adjacent(
                    Location {
                        x: i as i32,
                        y: j as i32,
                    },
                    &roll_data,
                ) {
                    // let row= roll_data.get_mut(i).expect("Expecting a row here");
                    // let mut value = row.get_mut(j).expect("Expecting a value here");
                    // value = &mut 'x';

                    roll_data[i][j] = 'x';
                    answer += 1;
                }
            }
        }
    }

    // Useful tool for looking at what is in a 2d vector!
    // dbg!(roll_data.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>());

    Some(Answer { answer })
}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");
    None
}

// Attempted answers

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
    let result1 = part1(&contents);
    println!("Part1 result {result1:?}");

    let doing_part2 = false;
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
        assert_eq!(result, Some(Answer { answer: 1395 }));
    }

    #[test]
    fn test_part1_example() {
        let contents = "..@@.@@@@.\n\
                                @@@.@.@.@@\n\
                                @@@@@.@.@@\n\
                                @.@@@@..@.\n\
                                @@.@@@@.@@\n\
                                .@@@@@@@.@\n\
                                .@.@.@.@@@\n\
                                @.@@@.@@@@\n\
                                .@@@@@@@@.\n\
                                @.@.@@@.@."
            .to_string();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 13 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        let contents = "987654321111111\n\
                              811111111111119\n\
                              234234234234278\n\
                              818181911112111"
            .to_string();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 357 }));
    }
}
