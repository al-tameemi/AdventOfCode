use std::fs;
#[macro_use] extern crate scan_fmt;

fn initialize_stacks(crates_data: Vec<&str>) -> Vec<Vec<char>> {
    let mut crates_data = crates_data;

    let crates_count = crates_data[crates_data.len() - 1].trim().split(' ').filter(|character| !character.is_empty() ).count();
    crates_data.pop();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); crates_count];

    for line in crates_data.iter().rev() {
        for i in (1..line.len()).step_by(4) {
            let j = (i + 3) / 4 - 1;
            let char = line.chars().nth(i).unwrap();
            if char != ' ' {
                stacks[j].push(char);
            }
        }
    }

    stacks
}

fn main() {
    let binding = fs::read_to_string("input").unwrap();
    let file = binding.split("\n\n").collect::<Vec<&str>>();
    let crates_data = file[0].split("\n").collect::<Vec<&str>>();
    let moves = file[1].split("\n");

    let mut stacks = initialize_stacks(crates_data);
    let stacks_copy = stacks.clone();

    for a_move in moves.clone() {
        let (count, from, to) = scan_fmt!(a_move, "move {} from {} to {}", i32, usize, usize).unwrap();
        let from = from - 1;
        let to = to - 1;
        for _ in 0..count {
            let out = stacks[from].pop().unwrap();
            stacks[to].push(out);
        }
    }

    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();


    let mut stacks = stacks_copy;
    for a_move in moves {
        let (count, from, to) = scan_fmt!(a_move, "move {} from {} to {}", usize, usize, usize).unwrap();
        let from = from - 1;
        let to = to - 1;

        let mut out: Vec<char> = stacks[from][(stacks[from].len() - count) ..].to_vec();
        stacks[from] = stacks[from][..(stacks[from].len() - count)].to_vec();
        stacks[to].append(&mut out);
    }

    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}
