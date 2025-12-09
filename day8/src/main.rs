#![feature(core_float_math)]
use core::f32::math::sqrt;
use std::{cmp::Ordering, fmt, fs::{self, File}};
use std::io::prelude::*;

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
struct Location {
    x: i64,
    y: i64,
    z: i64,
}

impl Location {
    fn calculate_distance(&self, other_location: &Location) -> f32 {
        let x_diff = self.x - other_location.x;
        let y_diff = self.y - other_location.y;
        let z_diff = self.z - other_location.z;
        return sqrt((x_diff.pow(2) + y_diff.pow(2) + z_diff.pow(2)) as f32);
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct LocationPair {
    loc1: Location,
    loc2: Location,
}

impl LocationPair {
    fn calculate_distance(&self) -> f32 {
        return self.loc1.calculate_distance(&self.loc2);
    }

    fn get_locations(&self) -> Vec<Location> {
        return vec![self.loc1.clone(), self.loc2.clone()];
    }

    fn swap_locations(&self) -> Self {
        return LocationPair {
            loc1: self.loc2.clone(),
            loc2: self.loc1.clone(),
        };
    }
}

impl Ord for LocationPair {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.calculate_distance()).total_cmp(&other.calculate_distance())
    }
}

impl PartialOrd for LocationPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.calculate_distance()).total_cmp(&other.calculate_distance()))
    }
}

impl fmt::Debug for LocationPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocationPair")
            .field("\n  loc1", &self.loc1)
            .field("\n  loc2", &self.loc2)
            .field("\n  .calculate_distance()", &self.calculate_distance())
            .finish()
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct LocationCluster {
    locations: Vec<Location>,
    // Pairs is mainly there to assist with debugging where the locations from from
    pairs: Vec<LocationPair>,
}

impl LocationCluster {
    fn check_connected(&self, pair: &LocationPair) -> bool {
        return self.locations.contains(&pair.loc1) || self.locations.contains(&pair.loc2);
    }

    fn add_pair(&mut self, pair: &LocationPair) {
        for location in &pair.get_locations() {
            if !self.locations.contains(location) {
                self.locations.push(location.clone());
            }
        }
        self.pairs.push(pair.clone());
    }

    fn combine(&mut self, cluster: &LocationCluster) {
        for location in &cluster.locations {
            if !self.locations.contains(location) {
                self.locations.push(location.clone());
            }
        }
        self.pairs.extend(cluster.pairs.clone());
    }
}

impl fmt::Debug for LocationCluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocationCluster")
            .field("locations.len()", &self.locations.len())
            .field("locations", &self.locations)
            .finish()
    }
}

fn parse_coords(line: &str) -> (i64, i64, i64) {
    let split_line: Vec<&str> = line.split(",").collect();

    return (
        split_line[0].parse().expect("Expected a value here"),
        split_line[1].parse().expect("Expected a value here"),
        split_line[2].parse().expect("Expected a value here"),
    );
}

fn parse_locations(contents: &String) -> Vec<Location> {
    let mut locations: Vec<Location> = vec![];
    for line in contents.lines() {
        let (x, y, z) = parse_coords(line);
        locations.push(Location { x, y, z })
    }
    return locations;
}

fn create_pairs(locations: &Vec<Location>) -> Vec<LocationPair> {
    // take only the x closest pairs, where x is a value that we can tune
    let max_closest_per_location = 10;
    let mut closest_pairs: Vec<LocationPair> = vec![];
    for location in locations {
        let mut this_location_pairs: Vec<LocationPair> = vec![];
        for other_location in locations {
            if location == other_location {
                continue;
            }

            this_location_pairs.push(LocationPair {
                loc1: location.clone(),
                loc2: other_location.clone(),
            });
        }

        this_location_pairs.sort();
        let only_closest_pairs = this_location_pairs[..max_closest_per_location].to_vec();
        closest_pairs.extend(only_closest_pairs)

    }
    closest_pairs.sort();

    return closest_pairs;
}

fn add_pairs_to_clusters(
    clusters: Vec<LocationCluster>,
    pair: &LocationPair,
) -> Vec<LocationCluster> {
    // Loop through each cluster in clusters to see which the pair belongs
    // If it belongs in multiple them combine the clusters

    let mut connected: Vec<LocationCluster> = vec![];
    let mut out_clusters: Vec<LocationCluster> = vec![];

    for cluster in &clusters {
        if cluster.check_connected(pair) {
            connected.push(cluster.clone())
        } else {
            out_clusters.push(cluster.clone())
        }
    }

    match connected.len() {
        0 => {
            // println!("New cluster for {pair:?}");
            out_clusters.push(LocationCluster {
                locations: pair.get_locations(),
                pairs: vec![pair.clone()],
            });
        }
        1 => {
            // println!("Add to cluster for {pair:?}");
            let cluster: &mut LocationCluster =
                connected.get_mut(0).expect("Expected a cluster here");
            cluster.add_pair(pair);
            // dbg!(&cluster);
            out_clusters.push(cluster.clone());
        }
        _ => {
            // println!("Combine clusters for {pair:?} {connected:?}");

            let mut new_combo_cluster = LocationCluster {
                locations: pair.get_locations(),
                pairs: vec![pair.clone()],
            };

            for cluster in &connected {
                new_combo_cluster.combine(cluster);
            }
            out_clusters.push(new_combo_cluster);
        }
    }

    return out_clusters;
}

fn remove_duplicates(closest_pairs: Vec<LocationPair>) -> Vec<LocationPair> {
    let mut out_pairs: Vec<LocationPair> = vec![];
    let pairs_len = closest_pairs.len();
    for (i, pair) in closest_pairs.iter().enumerate() {
        if i % 10000 == 0 {
            println!("Currently on pair {i}/{pairs_len}")
        }
        if !out_pairs.contains(&pair.swap_locations()) {
            out_pairs.push(pair.clone())
        }
    }
    return out_pairs;
}

fn get_three_largest_clusters(mut clusters: Vec<LocationCluster>) -> Vec<LocationCluster> {
    // let mut size_map: HashMap<&LocationCluster, usize> = HashMap::new();

    // for cluster in &clusters {
    //     size_map.insert(cluster, cluster.locations.len());
    // }

    // size_map.sor

    clusters.sort_by(|c1, c2| c1.locations.len().cmp(&c2.locations.len()));

    let out_clusters: Vec<LocationCluster> = clusters[clusters.len() - 3..].to_vec();

    return out_clusters;
}

fn _write_to_file(text: &String) -> std::io::Result<()> {
    let mut file = File::create("closest_pairs.txt")?;
    file.write_all(text.as_bytes())?;
    Ok(())
}


fn get_all_pairs(contents: &String) -> (Vec<Location>, Vec<LocationPair>) {
    println!("Parsing locations");
    let locations = parse_locations(contents);

    println!("Creating closest pairs");
    let mut closest_pairs = create_pairs(&locations);

    println!("Removing duplicates");
    closest_pairs = remove_duplicates(closest_pairs);

    println!("Writing debug to file");
    // let debug_string: String = "".to_string();
    // let debug_string: String = closest_pairs.iter().map(|p| format!("{p:?}\n")).collect();
    // let _ = write_to_file(&debug_string);


    dbg!(&closest_pairs);

    return (locations, closest_pairs);
}

fn part1(contents: &String, num_connections: usize) -> Option<Answer> {
    let (_, closest_pairs) = get_all_pairs(contents);
    let mut clusters: Vec<LocationCluster> = vec![];

    println!("Starting clustering");

    println!("Finding clusters with the closest {num_connections} connections");
    for pair in &closest_pairs[..num_connections] {
        println!("\nFinding cluster for {pair:?}");
        clusters = add_pairs_to_clusters(clusters, pair);
    }

    let three_largest_clusters: Vec<LocationCluster> = get_three_largest_clusters(clusters);

    let mut answer = three_largest_clusters
        .get(0)
        .expect("Expected a cluster here")
        .locations
        .len() as u64;

    dbg!(&three_largest_clusters);

    for cluster in &three_largest_clusters[1..] {
        answer *= cluster.locations.len() as u64;
    }

    Some(Answer { answer })
}

// Part 1 attempted answers

fn part2(contents: &String) -> Option<Answer> {

    let (locations, closest_pairs) = get_all_pairs(contents);

    let mut clusters = pre_add_locations(locations);

    println!("Starting clustering");

    println!("Finding required connections");
    let mut i = 0;
    let mut pair= None;
    while clusters.len() != 1  {
        pair = Some(&closest_pairs[i]);
        println!("\nFinding cluster for {pair:?}");
        clusters = add_pairs_to_clusters(clusters, pair.expect("Expected a pair here"));
        i += 1;
    }

    println!("Final pair was {pair:?} i was {i}");

    if pair.is_none() {
        panic!("Pair was not initialized");
    }

    let answer = (pair.expect("Expected a pair").loc1.x * pair.expect("Expected a pair").loc2.x) as u64;
    return Some(Answer{answer})
}

fn pre_add_locations(locations: Vec<Location>) -> Vec<LocationCluster> {
    let mut clusters = vec![];

    for location in locations {
        clusters.push(LocationCluster{locations: vec![location], pairs: vec![]})
    }

    return clusters;

}

// Part 2 attempted answers

fn main() {
    let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
    let do_part1 = false;
    let do_part2 = true;
    if do_part1 {
        let result1 = part1(&contents, 1000);
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
                contents: "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
                    .to_string(),
            }
        }
    }

    #[test]
    fn test_part1_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part1(&contents, 10);
        assert_eq!(result, Some(Answer { answer: 40 }));
    }

    #[ignore]
    #[test]
    fn test_part1_self_created() {
        let contents = "0,0,0
1,0,0
0,10,0
1,10,0
100,100,100
101,102,101
100,200,100
501,500,401
500,500,500
600,100,100
700,700,700
800,800,800
900,900,900
1000,1000,1000".to_string();
        let result = part1(&contents, 8);
        assert_eq!(result, Some(Answer { answer: 40 }));
    }

    #[test]
    fn test_part1() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part1(&contents, 1000);
        assert_eq!(result, Some(Answer { answer: 24360 }));
    }

    #[test]
    fn test_part2_example() {
        let setup = Setup::new();
        let contents = &setup.contents;
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 25272 }));
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let contents = LocalFileInputGetter { path: "input.txt" }.get_input();
        let result = part2(&contents);
        assert_eq!(result, Some(Answer { answer: 2341 }));
    }
}
