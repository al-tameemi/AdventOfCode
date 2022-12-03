use std::fs;
use itertools::sorted;

fn main() {
    let value: i32 = sorted(fs::read_to_string("input").unwrap().split("\n\n").map(|b| { b.split('\n').map(|g| { if !g.is_empty() { g.parse::<i32>().unwrap() } else { 0 } }).sum::<i32>() }).collect::<Vec<i32>>()).rev().take(3).sum();
    println!("value: {:#?}", value);
}
