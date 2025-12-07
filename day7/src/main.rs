use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
struct Answer {
    answer: u64
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
    
    let mut laser_locations: Vec<usize> = vec![];

    let start_location = find_initial_location(contents).expect("Expected the start location to be found");

    let mut split_count = 0;

    laser_locations.push(start_location);


    dbg!(&laser_locations);

    for line in &contents.lines().collect::<Vec<&str>>()[1..] {
        (laser_locations, split_count) = find_new_locations_part1(line, laser_locations, split_count);
        dbg!(&laser_locations);
    }
    // let answer = laser_locations.len() as u64;

   return Some(Answer{ answer: split_count as u64 })
}

fn find_new_locations_part1(line: &str, mut laser_locations: Vec<usize>, mut split_count: usize) -> (Vec<usize>, usize) {
    for (i, char) in line.chars().enumerate() {
        if char == '^' && laser_locations.contains(&i){
            // Count home many times &i is in there, and do this multiple times
            laser_locations.push(i-1);
            laser_locations.push(i+1);
            laser_locations.retain(|x| *x != i);
            split_count += 1;
        }
    }
    return (laser_locations, split_count)
}

fn find_new_locations_part2(line: &str, mut laser_locations: HashMap<usize, usize>, mut split_count: usize) -> (HashMap<usize, usize>, usize) {
    for (i, char) in line.chars().enumerate() {
        if char == '^' && laser_locations.contains_key(&i){
            // Count home many times &i is in there, and do this multiple times
            let location_amount = laser_locations[&i];
            laser_locations = increment_location(laser_locations, i-1, location_amount);
            laser_locations = increment_location(laser_locations, i+1, location_amount);
            laser_locations.remove(&i);
            split_count += 1;
        }
    }
    return (laser_locations, split_count)
}

fn find_initial_location(contents: &str) -> Option<usize> {
    for (i, c) in contents.chars().enumerate() {
        if c == 'S' {
            return Some(i)
        }
    } 
    None
}

// Part 1 attempted answers

fn part2(contents: &String) -> Option<Answer> {
    let mut laser_locations: HashMap<usize, usize> = HashMap::new();

    let start_location = find_initial_location(contents).expect("Expected the start location to be found");

    let mut split_count = 0;

    // laser_locations.push(start_location);

    laser_locations = increment_location(laser_locations, start_location, 1);

    dbg!(&laser_locations);

    for line in &contents.lines().collect::<Vec<&str>>()[1..] {
        (laser_locations, split_count) = find_new_locations_part2(line, laser_locations, split_count);
        dbg!(&laser_locations);
    }

    let mut answer: u64 = 0;

    for count in laser_locations.values() {
        answer += *count as u64;
    }

    return Some(Answer{ answer })

}

fn increment_location(mut laser_locations: HashMap<usize, usize>, location: usize, amount: usize) -> HashMap<usize, usize> {
    laser_locations.entry(location).and_modify(|counter| *counter += amount).or_insert(amount);
    return laser_locations
}


// Part 2 attempted answers
// 524 too low (expected, as also low on the test, but not sure why currently)


fn main() {

    let contents = LocalFileInputGetter{ path: "input.txt"}.get_input();
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
                contents: ".......S.......\n\
                           ...............\n\
                           .......^.......\n\
                           ...............\n\
                           ......^.^......\n\
                           ...............\n\
                           .....^.^.^.....\n\
                           ...............\n\
                           ....^.^...^....\n\
                           ...............\n\
                           ...^.^...^.^...\n\
                           ...............\n\
                           ..^...^.....^..\n\
                           ...............\n\
                           .^.^.^.^.^...^.\n\
                           ..............."
                .to_string(), 
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 21 }));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 1672 }));
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 40 }));
    }

    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 231229866702355 }));
    }
}