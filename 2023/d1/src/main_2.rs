use std::fs;

const VALID_NUMS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn main() {
    let sum = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let first = (0..line.len()).find_map(|i| is_num(bytes, i)).unwrap() * 10;
            let second = (0..line.len()).rev().find_map(|i| is_num(bytes, i)).unwrap();
            first + second
        })
        .sum::<usize>();

    println!("{sum}");
}

fn is_num(line: &[u8], i: usize) -> Option<usize> {
    line[i]
        .is_ascii_digit()
        .then_some((line[i] - b'0') as usize)
        .or(VALID_NUMS
            .iter()
            .enumerate()
            .find(|(_, number_name)| line[i..].starts_with(number_name))
            .map(|(index, _)| index + 1))
}
