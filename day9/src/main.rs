extern crate pbr;

use std::{cmp::Ordering, collections::HashMap, fs};

use pbr::ProgressBar;

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

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Red,
    Green,
    Empty
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
    
    fn is_valid(&self, map: &HashMap<u64, HashMap<u64, Tile>>) -> bool {

        // Check all four corners as a quick first pass
        let corner1 = &self.loc1;
        let corner2 = &self.loc2;

        let corner3 = &TwoDimensionalLocation{x: self.loc1.x, y: self.loc2.y};
        let corner4 = &TwoDimensionalLocation{x: self.loc2.x, y: self.loc1.y};

        let corners = [corner1, corner2, corner3, corner4];

        for corner in corners {
            if !check_location_in_loop(map, corner) {
                return false;
            }
        }

        println!("This is possibly valid, now checking every spot to be 100% sure");
        // Check every outer value of the square made by the pair, if map shows red or green for it then it is valid, otherwise its not valid
        let (start_x, end_x) = sort_values(self.loc1.x as u64, self.loc2.x as u64);
        let (start_y, end_y) = sort_values(self.loc1.y as u64, self.loc2.y as u64);

        println!("Starting row 1");
        if !row_is_valid(&map, start_x, end_x, start_y) {
            return false
        }
        println!("Starting row 2");
        if !row_is_valid(&map, start_x, end_x, end_y) {
            return false
        }

        println!("Starting column 1");
        if !column_is_valid(&map, start_y, end_y, start_x) {
            return false
        }

        println!("Starting column 2");
        if !column_is_valid(&map, start_y, end_y, end_x) {
            return false
        }

        return true
    }
}

fn column_is_valid(map: &HashMap<u64, HashMap<u64, Tile>>, start_y: u64, end_y: u64, x: u64) -> bool {

    let mut pb = ProgressBar::new(end_y-start_y);
    pb.format("╢▌▌░╟");
    for j in start_y..end_y+1 {
        pb.inc();
        if !check_in_loop(&map, x, j) {
            return false;
        }
    }
    pb.finish_print("done");
    return true;
}

fn row_is_valid(map: &HashMap<u64, HashMap<u64, Tile>>, start_x: u64, end_x: u64, y: u64) -> bool {
    let mut pb = ProgressBar::new(end_x-start_x);
    pb.format("╢▌▌░╟");
    for i in start_x..end_x+1 {
        pb.inc();
        if !check_in_loop(&map, i, y) {
            return false;
        }
    }
    pb.finish_print("done");
    return true;
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

fn default_map(x: u64, val: Tile) -> HashMap<u64, Tile> {
    let mut map: HashMap<u64, Tile> = HashMap::new();

    map.insert(x, val);

    return map;
}

fn set_map_location(mut map: HashMap<u64, HashMap<u64, Tile>>, x: u64, y: u64, val: Tile) -> HashMap<u64, HashMap<u64, Tile>>  {
    map.entry(y)
        .and_modify(|x_map| {x_map.insert(x, val.clone());})
        .or_insert(default_map(x, val));

    return map
}

fn get_map_location(map: &HashMap<u64, HashMap<u64, Tile>>, x: u64, y: u64) -> Tile {
    let empty_map = HashMap::new();
    let x_map = map.get(&y).unwrap_or(&empty_map);

    let x_val = x_map.get(&x).unwrap_or(&Tile::Empty);
    return x_val.clone()
}

fn check_location_in_loop(map: &HashMap<u64, HashMap<u64, Tile>>, location: &TwoDimensionalLocation) -> bool {
    return check_in_loop(map, location.x as u64, location.y as u64)
}

fn check_in_loop(map: &HashMap<u64, HashMap<u64, Tile>>, x: u64, y: u64) -> bool {
    // Either the current location needs to be a red or green tile (not empty)
    // Or there needs to be a red or green tile in all 4 cardinal directions.

    if get_map_location(map, x, y) != Tile::Empty {
        return true;
    }

    let mut y_map_keys: Vec<&u64> = map.keys().collect();

    y_map_keys.sort();
    let min_y = y_map_keys[0];

    y_map_keys.reverse();
    let max_y = y_map_keys[0];

    if min_y <= &y && max_y >= &y && check_left(map, x, y) && check_right(map, x, y) {
        return true;
    }

    return false;
}

fn check_left(map: &HashMap<u64, HashMap<u64, Tile>>, x: u64, y: u64) -> bool {
    let empty_map = &HashMap::new();
    let x_map = map.get(&y).unwrap_or(empty_map);
    let mut x_map_keys: Vec<&u64> = x_map.keys().collect();

    x_map_keys.sort();
    let min_x = x_map_keys[0];

    if min_x <= &x {
        return true;
    }
    println!("Left was false");
    return false
}

fn check_right(map: &HashMap<u64, HashMap<u64, Tile>>, x: u64, y: u64) -> bool {
    let empty_map = &HashMap::new();
    let x_map = map.get(&y).unwrap_or(empty_map);
    let mut x_map_keys: Vec<&u64> = x_map.keys().collect();

    x_map_keys.sort();
    x_map_keys.reverse();
    let max_x = x_map_keys[0];

    if max_x >= &x {
        return true;
    }
    println!("Right was false for {x}, {y}, max_x is {max_x}");
    return false
}


fn generate_map(contents: &str) -> HashMap<u64, HashMap<u64, Tile>> {
    // Map contains location - 1 index with red or green for tile colour
    let mut map: HashMap<u64, HashMap<u64, Tile>> = HashMap::new();

    // Add green tile outline
    let lines: Vec<&str> = contents.lines().collect();
    let (mut x1, mut y1) = split_line(lines[lines.len() - 1]);

    for line in &lines {
        let (x2, y2) = split_line(line);
        // Do generation

        let (start_x, end_x) = sort_values(x1, x2);
        let (start_y, end_y) = sort_values(y1, y2);


        // println!("x: {start_x}+1..{end_x}+1 and {start_y}+1..{end_y}+1");
        for i in start_x..end_x + 1 {
            for j in start_y..end_y + 1 {
                // println!("Outputting i {i} outputting j {j}");
                if get_map_location(&map, j, i) != Tile::Red {
                    map = set_map_location(map, i, j, Tile::Green);
                }
            }
        }

        map = set_map_location(map, start_x, start_y, Tile::Red);
        map = set_map_location(map, end_x, end_y, Tile::Red);

        (x1, y1) = (x2, y2)
    }

    // Add red tiles
    // for line in lines {
    //     let (x, y) = split_line(line);
    //             map = set_map_location(map, x, y, Tile::Red);
    // }

    // dbg!(&map);



    return map;
}

fn sort_values(v1: u64, v2: u64) -> (u64, u64) {
    if v1 > v2 {
        return (v2, v1)
    }
    return (v1, v2)
}


fn split_line(line: &str) -> (u64, u64) {
    let split_line: Vec<&str> = line.split(",").collect();

    let x = split_line[0].parse().expect("Expected a value here");
    let y = split_line[1].parse().expect("Expected a value here");

    return (x, y);
}

fn create_display_map(map: &HashMap<u64, HashMap<u64, Tile>>) -> String {

    let mut out_string = "".to_string();

    let max_y = 9;
    let max_x = 14;


    for i in 0..max_y {
        let row: Option<&HashMap<u64, Tile>> = map.get(&i);
        for j in 0..max_x {
            if row.is_none() {
                out_string += ".";
                continue
            }
            let val = row.unwrap().get(&j);

            match val {
                Some(Tile::Red) => out_string += "#",
                Some(Tile::Green) => out_string += "X",
                _ => out_string += "."
            }

        }
        out_string += "\n";
    }
    return out_string

}

fn part2(contents: &String) -> Option<Answer> {
    // First generate map of all "in" and "out" positions
    println!("Generating map...");
    let map: HashMap<u64, HashMap<u64, Tile>> = generate_map(contents);

    println!("Display map...");
    let out_string = create_display_map(&map);
    println!("{out_string}");

    println!("Parsing locations...");
    // Generate all the pairs, same as part1
    let locations: Vec<TwoDimensionalLocation> = parse_locations(contents);

    println!("Creating pairs...");
    let mut pairs: Vec<TwoDimensionalLocationPair> = create_pairs(locations);

    pairs.sort();
    pairs.reverse();

    let count = pairs.len();

    let mut pb = ProgressBar::new(count as u64);
    pb.format("╢▌▌░╟");

    // Check the largest is valid, try next down until a valid one is found
    for (i, pair) in pairs.iter().enumerate() {
        println!("Checking pair {pair:?}... {i}/{count}");
        pb.inc();
        if pair.is_valid(&map) {
            let answer = pair.calculate_square_size() as u64;
            pb.finish_print("done");
            return Some(Answer { answer });

        }
    }



    // Output largest valid square
    return None
}


// Part 2 attempted answers
// 4474437111 (2290)

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

    #[test]
    fn test_part2_generate_map() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let map: HashMap<u64, HashMap<u64, Tile>> = generate_map(contents);

        let display_map= create_display_map(&map);


        let expected_map_pre_fill = "..............
.......#XXX#..
.......X...X..
..#XXXX#...X..
..X........X..
..#XXXXXX#.X..
.........X.X..
.........#X#..
..............
";

        let lines_expected = expected_map_pre_fill.lines().collect::<Vec<&str>>();
        let first_line_expected = lines_expected[0];

        let lines_actual = display_map.lines().collect::<Vec<&str>>();
        let first_line_actual = lines_actual[0];

        dbg!(&lines_actual);

        assert_eq!(lines_actual.len(), lines_expected.len());

        assert_eq!(first_line_actual.len(), first_line_expected.len());

        assert_eq!(expected_map_pre_fill, display_map);
    }


    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}
