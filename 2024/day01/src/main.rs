use std::collections::HashMap;

fn part1() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut total: u64 = 0;
    let mut path1: Vec<u32>;
    let mut path2: Vec<u32>;

    (path1, path2) = data
        .lines()
        .map(|line| {
            let mut line_data = line.split("   ");
            (
                line_data.next().unwrap().parse::<u32>().unwrap(),
                line_data.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    path1.sort();
    path2.sort();

    for i in 0..path1.len() {
        total += (path1[i].abs_diff(path2[i])) as u64;
    }

    println!("{total}");
}

fn part2() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut total: u64 = 0;
    let mut path1: Vec<u32>;
    let mut path2: Vec<u32>;
    let mut path2_map: HashMap<u32, u32> = HashMap::new();

    (path1, path2) = data
        .lines()
        .map(|line| {
            let mut line_data = line.split("   ");
            (
                line_data.next().unwrap().parse::<u32>().unwrap(),
                line_data.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    path1.sort();
    path2.sort();

    for element in path2 {
        if !path2_map.contains_key(&element) {
            path2_map.insert(element, 0);
        }

        (*path2_map.get_mut(&element).unwrap()) += 1;
    }

    for element in path1 {
        if path2_map.contains_key(&element) {
            total += (element * path2_map[&element]) as u64;
        }
    }

    println!("{total}");
}

fn main() {
    part1();
    part2();
}
