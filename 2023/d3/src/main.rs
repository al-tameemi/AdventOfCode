use d3::*;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let map = include_bytes!("../input");
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;

    let total: usize = (0..map.len())
        .filter(|index: &usize| is_first_digit(map, *index))
        .map(|first_digit_index| {
            let last_digit_index = last_digit_index(map, first_digit_index);
            (
                first_digit_index,
                last_digit_index,
                into_number(map, first_digit_index, last_digit_index),
            )
        })
        .filter(|(first_digit_index, last_digit_index, _number)| {
            bounding_box_offsets(width, last_digit_index - first_digit_index).any(|offset| {
                map.get((*first_digit_index as isize + offset) as usize)
                    .map_or(false, |byte| byte != &b'.' && byte.is_ascii_punctuation())
            })
        })
        .map(|(_, _, number)| number)
        .sum();

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("{}", total);
}
