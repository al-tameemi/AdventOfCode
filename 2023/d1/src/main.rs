use std::fs;

struct MyStruct {
    data: u32,
}

impl MyStruct {
    fn from_str() -> MyStruct {
        MyStruct { data: 0 }
    }
}

fn main() {
    let my = MyStruct::from_str();

    let sum = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let mut first: char = '0';
            let mut second: char = '0';

            for char in line.chars() {
                if char.is_numeric() {
                    first = char;
                    break;
                }
            }

            for char in line.chars().rev() {
                if char.is_numeric() {
                    second = char;
                    break;
                }
            }

            let num = format!("{}{}", first, second);
            num.parse::<i32>().unwrap()
        })
        .sum::<i32>();
    println!("{sum}");
}
