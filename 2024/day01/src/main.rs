fn convert_str_to_num(c: &u8) -> usize {
    (c - '0' as u8) as usize
}

fn part1() -> usize {
    let data = include_bytes!("../input.txt");
    let mut path1: Vec<usize>;
    let mut path2: Vec<usize>;

    (path1, path2) = data
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                line[..5]
                    .iter()
                    .map(convert_str_to_num)
                    .enumerate()
                    .fold(0, |acc, (index, number)| {
                        acc + number * 10_u32.pow(4 - index as u32) as usize
                    }),
                line[8..]
                    .iter()
                    .map(convert_str_to_num)
                    .enumerate()
                    .fold(0, |acc, (index, number)| {
                        acc + number * 10_u32.pow(4 - index as u32) as usize
                    }),
            )
        })
        .unzip();

    path1.sort_unstable();
    path2.sort_unstable();

    path1
        .iter()
        .zip(path2)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<usize>()
}

fn part2() -> usize {
    let data = include_bytes!("../input.txt");
    let mut path1: Vec<usize> = Vec::new();
    let mut path2 = [0; 100_000];

    for line in data.split(|&b| b == b'\n') {
        path1.push(
            line[..5]
                .iter()
                .map(convert_str_to_num)
                .enumerate()
                .fold(0, |acc, (index, number)| {
                    acc + number * 10_u32.pow(4 - index as u32) as usize
                }),
        );
        path2[line[8..]
            .iter()
            .map(convert_str_to_num)
            .enumerate()
            .fold(0, |acc, (index, number)| {
                acc + number * 10_u32.pow(4 - index as u32) as usize
            })] += 1;
    }

    path1.into_iter().map(|n| path2[n] * n).sum::<usize>()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let value = part1();
    let elapsed = now.elapsed();
    println!("Part 1: {value}");
    println!("Elapsed: {:.5?}", elapsed);
    let now = Instant::now();
    let value = part2();
    let elapsed = now.elapsed();
    println!("Part 2: {value}");
    println!("Elapsed: {:.5?}", elapsed);
}
