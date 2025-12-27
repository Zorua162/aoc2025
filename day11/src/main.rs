use std::{collections::HashMap, fs};

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

#[derive(Debug, Clone)]
struct Node {
    input: String,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct NodeData {
    node_name: String,
    dac: bool,
    fft: bool,
    count: u64,
}

// impl Node {
// }

fn parse_input(contents: &String) -> Vec<Node> {
    let mut nodes = vec![];
    for line in contents.lines() {
        let split_line: Vec<&str> = line.split(": ").collect();
        let input = split_line.get(0).expect("Expected the input").to_string();

        let outputs: Vec<String> = split_line
            .get(1)
            .expect("Expected the output")
            .split(" ")
            .map(|i| i.to_string())
            .collect();

        nodes.push(Node { input, outputs })
    }

    return nodes;
}

fn part1(contents: &String) -> Option<Answer> {
    let nodes = parse_input(contents);

    let mut node_map = HashMap::new();

    for node in nodes.clone() {
        node_map.insert(node.input.clone(), node);
    }

    let first_node = node_map["you"].clone();

    let mut path_locations = vec![first_node];

    let mut not_all_out = true;

    let mut count = 0;

    while not_all_out {
        count += 1;
        dbg!(count);
        path_locations = find_next_nodes(node_map.clone(), path_locations);

        not_all_out = check_all_out(&path_locations)
    }

    let answer = path_locations.len() as u64;
    return Some(Answer { answer });
}

fn check_all_out(path_locations: &Vec<Node>) -> bool {
    for node in path_locations {
        if node.input != "out" {
            return true;
        }
    }
    return false;
}

fn find_next_nodes(node_map: HashMap<String, Node>, path_locations: Vec<Node>) -> Vec<Node> {
    // println!("Starting on new iteration with path_locations {path_locations:?}");
    let mut new_path_locations = vec![];
    for old_node in &path_locations {
        if old_node.input == "out" {
            new_path_locations.push(old_node.clone());
            continue;
        }
        for node_name in old_node.outputs.clone() {
            // println!("Node name is {node_name}");
            if node_name == "out" {
                new_path_locations.push(Node {
                    input: "out".to_string(),
                    outputs: vec![],
                });
            } else {
                let new_node = node_map
                    .get(&node_name)
                    .expect("Expected a node here")
                    .clone();
                new_path_locations.push(new_node);
            }
        }
    }

    return new_path_locations;
}

// Part 1 attempted answers
// 662: Correct!

fn part2(contents: &String) -> Option<Answer> {
    let nodes = parse_input(contents);

    let mut node_map = HashMap::new();

    for node in nodes.clone() {
        node_map.insert(node.input.clone(), node);
    }

    dbg!(&node_map);

    let mut path_data = vec![];

    path_data.push(NodeData {
        count: 1,
        node_name: "svr".to_string(),
        fft: false,
        dac: false,
    });

    let mut not_all_out = true;

    let mut count = 0;

    while not_all_out {
        count += 1;
        dbg!(count);
        dbg!(&path_data);
        path_data = find_next_nodes_part2(node_map.clone(), path_data);

        not_all_out = check_all_out_part2(&path_data)
    }

    dbg!(&path_data);

    let out_node_index = find_node(&path_data, "out", true, true).expect("Expected a NodeData here");

    let out_node = &path_data[out_node_index];

    let answer = out_node.count as u64;
    return Some(Answer { answer });
}

fn find_node(path_data: &Vec<NodeData>, node_name: &str, dac: bool, fft: bool) -> Option<usize> {
    for (i, node_data) in path_data.iter().enumerate() {
        if node_data.node_name == node_name && node_data.dac == dac && node_data.fft == fft {
            return Some(i);
        }
    }

    return None
}

fn check_all_out_part2(path_data: &Vec<NodeData>) -> bool {
    for node_data in path_data {
        if node_data.node_name != "out" && node_data.count != 0 {
            // One is not all out
            return true;
        }
    }
    // All are out
    return false;
}

fn find_next_nodes_part2(
    node_map: HashMap<String, Node>,
    path_data: Vec<NodeData>,
) -> Vec<NodeData> {
    let mut new_locations = vec![];

    for old_node_data in &path_data {
        let node_name = &old_node_data.node_name;
        println!("Node name is {node_name}");
        let out_names= vec!["out".to_string()];
        let node_new_locations;
        if node_name == "out" {
            node_new_locations = &out_names;
        } else {
            node_new_locations = &node_map[node_name].outputs;
        }
        // Add the out location count, and status of dac and fft to the new locations
        // If they ARE dac or fft then update them to show fft and/ or dac were passed

        let mut dac = old_node_data.dac;
        let mut fft = old_node_data.fft;

        for new_location_name in node_new_locations {
            if new_location_name == "dac" {
                dac = true;
            }
            if new_location_name == "fft" {
                fft = true;
            }
            let existing_new_index = find_node(&new_locations, new_location_name, dac, fft);
            match existing_new_index {
                None => new_locations.push(NodeData {
                    node_name: new_location_name.to_string(),
                    dac,
                    fft,
                    count: old_node_data.count,
                }),
                Some(index) => {
                    let node = new_locations.get_mut(index).expect("Expected a node here");
                    node.count += old_node_data.count;
                }
            }
        }
    }

    return new_locations;
}

// Part 2 attempted answers
// 297666180873600 too low
// 429399933071120 - correct!

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
        contents2: String,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                contents: "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
                    .to_string(),
                contents2: "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
                    .to_string(),
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 5 }));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents);
        assert_eq!(result, Some(Answer { answer: 662 }));
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents2;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}
