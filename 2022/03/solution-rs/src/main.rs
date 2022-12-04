use std::{fs, str::Split};

trait Priority {
    fn priority(self) -> i32;
}

impl Priority for char {
    fn priority(self) -> i32 {
        if (self as i32) < 91 {
            return (self as i32) - 38;
        } else {
            return (self as i32) - 96;
        }
    }
}

fn main() {
    let binding = fs::read_to_string("input").unwrap();
    let data: Split<&str> = binding.split("\n");
    let priority = data.clone().map(|rucksack| {
                                for element in rucksack[.. (rucksack.len() / 2)].chars() {
                                    for second_element in rucksack[(rucksack.len() / 2) ..].chars() {
                                        if second_element == element {
                                            return element.priority();
                                        }
                                    }
                                }
                                0
                            })
                            .sum::<i32>();

    println!("{}", priority);

    let binding = data.clone().collect::<Vec<&str>>();
    let result = binding.chunks(3)
                            .map(|group| {
                                for char1 in group[0].chars() {
                                    for char2 in group[1].chars() {
                                        for char3 in group[2].chars() {
                                            if char1 == char2 && char2 == char3 {
                                                return char1.priority();
                                            }
                                        }
                                    }
                                }
                                0
                            })
                            .sum::<i32>();

    println!("{:#?}", result);
}
