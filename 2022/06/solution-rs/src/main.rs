use std::fs;

fn main() {
    let data = fs::read_to_string("input").unwrap();
    let chars = data.chars().collect::<Vec<char>>();
    let mut breaking = false;
    for i in 0..(data.len() - 4) {
        if breaking {
            break;
        }
        let mut list = Vec::new();
        for j in 0..4 {
            if !list.contains(&chars[i + j]) {
                list.push(chars[i + j].clone());
            } else {
                break;
            }
            if j == 3 {
                println!("{}", i + j + 1);
                breaking = true;
                break;
            }
        }
    }

    breaking = false;

    for i in 0..(data.len() - 14) {
        if breaking {
            break;
        }
        let mut list = Vec::new();
        for j in 0..14 {

            if !list.contains(&chars[i + j]) {
                list.push(chars[i + j].clone());
            } else {
                break;
            }
            if j == 13 {
                println!("{}", i + j + 1);
                breaking = true;
                break;
            }
        }
    }
}
