use std::fs;

fn main() {
    let predicate = |character: &char| character.is_ascii_digit();
    let sum = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let first = (line.chars().find(predicate).unwrap() as u32 - ('0' as u32)) * 10;
            let second = line.chars().rev().find(predicate).unwrap() as u32 - ('0' as u32);
            first + second
        })
        .sum::<u32>();

    println!("{sum}");
}
