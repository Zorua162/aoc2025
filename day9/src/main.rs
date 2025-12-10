use std::{cmp::Ordering, collections::HashMap, fs};

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

#[derive(Eq, PartialEq, Clone)]
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

fn create_pairs(locations: Vec<TwoDimensionalLocation>) -> Vec<TwoDimensionalLocationPair> {
    let mut out_pairs: Vec<TwoDimensionalLocationPair> = vec![];
    for location in &locations {
        for other_location in &locations {
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

    let mut pairs: Vec<TwoDimensionalLocationPair> = create_pairs(locations);

    pairs.sort();

    let answer = pairs[pairs.len() - 1].calculate_square_size() as u64;

    return Some(Answer { answer });
}

// Part 1 attempted answers

fn generate_map(contents: &str) -> HashMap<u64, HashMap<u64, bool>> {
    // True in the map means that there is a red or green tile there
    // Everywhere else defaults to false

    let mut map: HashMap<u64, HashMap<u64, bool>> = HashMap::new();

    // First draw the shape
    let lines: Vec<&str> = contents.lines().collect();
    let (mut x1, mut y1) = split_line(lines[lines.len() - 1]);
    for line in lines {
        let (x2, y2) = split_line(line);
        // Do generation

        let (start_x, end_x) = sort_values(x1, x2);
        let (start_y, end_y) = sort_values(y1, y2);

        for i in start_x..end_x + 1 {
            for j in start_y..end_y + 1 {
                println!("Outputting i {i} outputting j {j}");
                map.entry(i)
                    .and_modify(|x_map| {x_map.insert(j, true);})
                    .or_insert(HashMap::new());
            }
        }

        (x1, y1) = (x2, y2)
    }

    // Then flood fill it
    map = flood_fill(map);

    return map;
}

fn sort_values(v1: u64, v2: u64) -> (u64, u64) {
    if v1 > v2 {
        return (v2, v1)
    }
    return (v1, v2)
}

fn flood_fill(mut map: HashMap<u64, HashMap<u64, bool>>) -> HashMap<u64, HashMap<u64, bool>> {
    for row in map.values_mut() {

        if row.len() == 0 {
            continue
        }

        let mut row_values: Vec<&u64> = row.keys().collect();
        row_values.sort();

        // Start with false, as we know first row value will flip this to true
        let mut inside = false;

        println!("row_values {row_values:?}");


        for i in *row_values[0]..*row_values[row_values.len()-1] {

            // If its already true, then that means this is a wall, so flip to off if we were already inside
            let val = row.get(&i);

            if val.is_none() {
                continue
            }
            if *val.unwrap() {
                inside = !inside;
            }

            if inside {
                row.insert(i, true);
            }

        }

    }
    return map;

}

fn split_line(line: &str) -> (u64, u64) {
    let split_line: Vec<&str> = line.split(",").collect();

    let x = split_line[0].parse().expect("Expected a value here");
    let y = split_line[1].parse().expect("Expected a value here");

    return (x, y);
}

fn display_map(map: HashMap<u64, HashMap<u64, bool>>) {

    let mut out_string = "".to_string();

    let mut y_keys = map.keys().collect::<Vec<&u64>>();
    y_keys.sort();
    y_keys.reverse();

    let max_y = *y_keys[0];

    let mut max_x = 0;

    for row in map.values() {
        if row.len() == 0 {
            continue
        }
        let mut x_keys = row.keys().collect::<Vec<&u64>>();
        x_keys.sort();
        x_keys.reverse();
        let row_max = *x_keys[0];
        if row_max > max_x {
            max_x = row_max;
        }
    }


    for i in 0..max_y {
        let row: Option<&HashMap<u64, bool>> = map.get(&i);
        for j in 0..max_x {
            if row.is_none() {
                out_string += ".";
                continue
            }
            let val = row.unwrap().get(&j);

            if let Some(true) = val {
                out_string += "#";

            } else {
                out_string += ".";
            }
        }
        out_string += "\n";
    }

    println!("{out_string}")
}

fn part2(contents: &String) -> Option<Answer> {
    // First generate map of all "in" and "out" positions
    let map: HashMap<u64, HashMap<u64, bool>> = generate_map(contents);

    display_map(map);

    // Generate all the pairs, same as part1

    // Do a validity check on the square for each pair, remove non-valid squares using map

    // Same as part1 now with sort by size

    // Output largest valid square

    None
}


// Part 2 attempted answers

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
    let do_part1 = true;
    let do_part2 = false;
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
