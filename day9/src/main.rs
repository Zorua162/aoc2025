use std::{cmp::Ordering, fs};

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

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct TwoDimensionalLocation {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct TwoDimensionalLocationPair {
    loc1: TwoDimensionalLocation,
    loc2: TwoDimensionalLocation,
}

impl TwoDimensionalLocationPair {
    fn calculate_square_size(&self) -> i64 {
        let x_diff: i64 = self.loc1.x - (self.loc2.x + 1).abs();
        let y_diff: i64 = self.loc1.y - (self.loc2.y + 1).abs();
        return x_diff * y_diff;
    }
}

impl PartialOrd for TwoDimensionalLocationPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.calculate_square_size()).partial_cmp(&other.calculate_square_size())
    }
}

impl Ord for TwoDimensionalLocationPair {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.calculate_square_size())
            .partial_cmp(&other.calculate_square_size())
            .expect("Expected an Ordering here")
    }
}

fn parse_locations(contents: &String) -> Vec<TwoDimensionalLocation> {
    let mut locations: Vec<TwoDimensionalLocation> = vec![];

    for line in contents.lines() {
        let split: Vec<&str> = line.split(",").collect();
        locations.push(TwoDimensionalLocation {
            x: split[0].parse().expect("Expected a value here"),
            y: split[1].parse().expect("Expected a value here"),
        })
    }

    return locations;
}

fn create_pairs(locations: &Vec<TwoDimensionalLocation>) -> Vec<TwoDimensionalLocationPair> {
    let mut out_pairs: Vec<TwoDimensionalLocationPair> = vec![];
    for location in locations {
        for other_location in locations {
            if location == other_location {
                continue;
            }

            out_pairs.push(TwoDimensionalLocationPair {
                loc1: location.clone(),
                loc2: other_location.clone(),
            })
        }
    }

    return out_pairs;
}

fn part1(contents: &String) -> Option<Answer> {
    let locations: Vec<TwoDimensionalLocation> = parse_locations(contents);

    let mut pairs: Vec<TwoDimensionalLocationPair> = create_pairs(&locations);

    pairs.sort();
    pairs.reverse();

    let answer = pairs[0].calculate_square_size() as u64;

    return Some(Answer { answer });
}

// Part 1 attempted answers

fn part2(contents: &String) -> Option<Answer> {
    println!("Parsing locations...");
    // Generate all the pairs, same as part1
    let locations: Vec<TwoDimensionalLocation> = parse_locations(contents);

    println!("Creating pairs...");
    // let mut pairs: Vec<TwoDimensionalLocationPair> = create_part2_pairs(locations);
    let mut pairs: Vec<TwoDimensionalLocationPair> = create_pairs(&locations);


    let total_pairs = pairs.len();
    println!("There were {total_pairs} total pairs");

    pairs = filter_pairs(&locations, pairs);

    // Output largest valid square

    pairs.sort();
    pairs.reverse();

    let num_pairs = pairs.len();
    
    println!("There were {num_pairs} valid pairs");

    for pair in &pairs[..9] {
        let size = pair.calculate_square_size();
        println!("For pair {pair:?} size is {size}")
    }

    let answer = pairs[0].calculate_square_size() as u64;

    return Some(Answer { answer });
}

fn filter_pairs(
    locations: &Vec<TwoDimensionalLocation>,
    pairs: Vec<TwoDimensionalLocationPair>,
) -> Vec<TwoDimensionalLocationPair> {
    let mut out_pairs = vec![];
    for pair in pairs {
        // let pair_size = pair.calculate_square_size();
        // println!("Checking pair {pair:?} with size {pair_size}");

        if check_valid_pair(&pair, locations) {
            out_pairs.push(pair);
        }
    }
    return out_pairs;
}

fn check_valid_pair(
    pair: &TwoDimensionalLocationPair,
    locations: &[TwoDimensionalLocation],
) -> bool {
    for location in locations {
        if location == &pair.loc1 || location == &pair.loc2 {
            continue;
        }

        if location_inside_pair(location, pair) {
            // println!("{pair:?} is invalid due to {location:?}");
            return false;
        }
    }
    return true;
}

fn location_inside_pair(
    location: &TwoDimensionalLocation,
    pair: &TwoDimensionalLocationPair,
) -> bool {
    let (low_x, high_x) = sort_values(pair.loc1.x, pair.loc2.x);
    let (low_y, high_y) = sort_values(pair.loc1.y, pair.loc2.y);
    if location.x <= low_x || location.x > high_x {
        return false;
    }

    if location.y <= low_y || location.y > high_y {
        return false;
    }

    return true;
}

fn sort_values(v1: i64, v2: i64) -> (i64, i64) {
    if v1 > v2 {
        return (v2, v1);
    }
    return (v1, v2);
}

#[allow(dead_code)]
fn create_part2_pairs(
    mut locations: Vec<TwoDimensionalLocation>,
) -> Vec<TwoDimensionalLocationPair> {
    // Instead of pairs we look at the triples that are next to each other
    // And only every other triple is valid.

    let mut pairs: Vec<TwoDimensionalLocationPair> = vec![];

    locations.reverse();

    let len_locations = locations.len();
    let mut location1 = locations
        .get(len_locations - 2)
        .expect("Expected location 1");
    let mut location2 = locations
        .get(len_locations - 1)
        .expect("Expected location 2");

    let mut val = 1;

    for location3 in &locations {
        // Skip every other one
        val += 1;
        let pair = TwoDimensionalLocationPair {
            loc1: location1.clone(),
            loc2: location3.clone(),
        };
        if is_left_turn(location1, location2, location3) {
            let size = pair.calculate_square_size();
            println!("Adding pair {pair:?} with size {size}");
            pairs.push(pair);
        } else {
            println!("Skipping {val} {pair:?}");
        }

        location1 = location2;
        location2 = location3;
        println!("val is {val}");
    }
    return pairs;
}

fn is_left_turn(
    location1: &TwoDimensionalLocation,
    location2: &TwoDimensionalLocation,
    location3: &TwoDimensionalLocation,
) -> bool {
    // Its a left turn if the following:
    // Left
    // -diff1_x       -diff2_y
    // +diff1_x       +diff2_y
    // -diff1_y       -diff2_x
    // +diff1_y       -diff2_x

    let diff1_x = location1.x - location2.x;
    // let diff1_y = location1.y - location2.y;

    let diff2_x = location2.x - location3.x;
    let diff2_y = location2.y - location3.y;

    if diff1_x == 0 {
        // For both -diff1_y and +diff1_y they are left if -diff2_x
        if diff2_x < 0 {
            return true;
        }
    } else {
        // -x or +x
        if diff1_x < 0 && diff2_y < 0 {
            return true;
        } else if diff1_x > 0 && diff2_y > 0 {
            return true;
        }
    }

    // Otherwise it is a right turn
    return false;
}

// Part 2 attempted answers

// 192570426 too low
// 4474437111 too high

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
    let do_part1 = false;
    let do_part2 = true;
    if do_part1 {
        let result1 = part1(&contents);
        println!("Part1 result {result1:?}");
    }

    if do_part2 {
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
                contents: "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
                .to_string(),
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 50 }));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 4777816465 }));
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 24 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}
