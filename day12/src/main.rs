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

fn part1(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");

    let (shapes, boxes) = parse_contents(contents);

    let mut count = 0;

    let shape_tile_counts = get_shape_counts(&shapes);

    for line in boxes {
        if check_line(line, &shapes, &shape_tile_counts) {
            count += 1;
        }
    }

    return Some(Answer { answer: count });
}

fn get_shape_counts(shapes: &Vec<Vec<&str>>) -> Vec<u64> {
    let mut shape_tile_counts = vec![];
    for shape in shapes {
        let mut tile_count = 0;
        for line in shape {
            for char in line.chars() {
                if char == '#' {
                    tile_count += 1;
                }
            }
        }
        shape_tile_counts.push(tile_count);
    }
    return shape_tile_counts
}

fn check_line(line: &str, shapes: &Vec<Vec<&str>>, shape_tile_counts: &Vec<u64>) -> bool {
    let split: Vec<&str> = line.split(": ").collect();
    
    // dbg!(&split);

    dbg!(&line);
    let box_size: Vec<u64> = split[0].split("x").map(|i| i.to_string().parse().expect("Expected an integer here")).collect();

    // dbg!(&box_size);

    let total_tiles = box_size[0] * box_size[1];

    // dbg!(&total_tiles);

    let shape_list: Vec<u64> = split[1].split(" ").map(|i| i.parse().expect("Expected an integer here")).collect();

    // Do an initial check to be sure that the number of tiles in the given shapes could fit in the boxes, to check if its worth trying the combinations...

    let mut reqd_tile_count = 0;
    for (i, shape_count) in shape_list.iter().enumerate() {
        // reqd_tile_count += shape_count * shape_tile_counts[i];
        reqd_tile_count += shape_count * 9;
    }

    dbg!(&reqd_tile_count);
    dbg!(&total_tiles);

    if reqd_tile_count > total_tiles {
        return false
    }


    return true
}

fn parse_contents(contents: &str) -> (Vec<Vec<&str>>, Vec<&str>) {
    let split: Vec<&str> = contents.split("\n\n").collect();

    dbg!(&split);

    let len_split = split.len();

    let boxes: Vec<&str> = split[len_split - 1..][0].split("\n").collect();

    dbg!(&boxes);

    let shapes: Vec<Vec<&str>> = split[..len_split - 1]
        .to_vec()
        .iter()
        .map(|i| i[3..].split("\n").collect::<Vec<&str>>())
        .collect();

    dbg!(&shapes);

    return (shapes, boxes)
}

// Part 1 attempted answers
// 579 - Correct :(

fn part2(contents: &String) -> Option<Answer> {
    println!("Contents is {contents}");
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
                contents: "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
                    .to_string(),
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 2 }));
    }

    #[ignore]
    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 1395 }));
    }

    #[ignore]
    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 43 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}