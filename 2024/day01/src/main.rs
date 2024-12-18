fn convert_str_to_num(c: &u8) -> usize {
    (c - '0' as u8) as usize
}

fn part1() {
    let data = include_bytes!("../input.txt");
    let mut path1: Vec<usize>;
    let mut path2: Vec<usize>;

    (path1, path2) = data
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                line[..5].iter().map(convert_str_to_num).enumerate().fold(0, |acc, (index, number)| {
                    acc + number * 10_u32.pow(4 - index as u32) as usize
                }),
                line[8..].iter().map(convert_str_to_num).enumerate().fold(0, |acc, (index, number)| {
                    acc + number * 10_u32.pow(4 - index as u32) as usize
                })
            
            )
        })
        .unzip();

    path1.sort_unstable();
    path2.sort_unstable();

    println!("{}", path1.iter().zip(path2).map(|(a, b)| a.abs_diff(b)).sum::<usize>());
}

fn part2() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut path1: Vec<usize> = Vec::new();
    let mut path2 = [0; 100_000];

    for line in data.lines() {
        let mut line_data = line.split("   ");
        path1.push(line_data.next().unwrap().parse::<usize>().unwrap());
        path2[line_data.next().unwrap().parse::<usize>().unwrap()] += 1;
    }

    println!("{}", path1.into_iter().map(|n| path2[n] * n).sum::<usize>());
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        part1();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.5?}", elapsed);
    let now = Instant::now();

    {
        part2();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.5?}", elapsed);
}
