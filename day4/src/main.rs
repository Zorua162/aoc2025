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
                if val == &'@' {
                    roll_count += 1;
                }
            }
        }
    }

    return roll_count < 4;
}

fn remove_rolls(roll_data: Vec<Vec<char>>, answer: &mut i32) -> (Vec<Vec<char>>, i32) {
    let mut updated_roll_data = roll_data.clone();

    if let Some(first_line) = roll_data.iter().next() {
        for (i, _) in roll_data.iter().enumerate() {
            for (j, _) in first_line.iter().enumerate() {
                if roll_data[i][j] == '@'
                    && check_loc_adjacent(
                        Location {
                            x: i as i32,
                            y: j as i32,
                        },
                        &roll_data,
                    )
                {
                    // let row= roll_data.get_mut(i).expect("Expecting a row here");
                    // let mut value = row.get_mut(j).expect("Expecting a value here");
                    // value = &mut 'x';

                    updated_roll_data[i][j] = 'x';
                    *answer += 1;
                }
            }
        }
    }
    return (updated_roll_data, *answer);
}

fn part1(contents: &String) -> Option<Answer> {
    let mut answer = 0;

    let mut roll_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    (_, answer) = remove_rolls(roll_data, &mut answer);

    // Useful tool for looking at what is in a 2d vector!
    // dbg!(roll_data.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>());

    Some(Answer {
        answer: answer as usize,
    })
}

// Attempted answers

fn part2(contents: &String) -> Option<Answer> {
    let mut answer = 0;
    let mut last_answer = -1;
    let mut roll_data: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    while last_answer != answer {
        last_answer = answer;

        (roll_data, answer) = remove_rolls(roll_data, &mut answer);

        dbg!(
            roll_data
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<String>>()
        );
        println!("answer {answer} last_answer {last_answer}")
    }

    Some(Answer {
        answer: answer as usize,
    })
}

// Attempted answers

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();

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

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 1395 }));
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 43 }));
    }

    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 8451 }));
    }
}
