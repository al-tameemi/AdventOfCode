#[inline(always)]
fn is_first_digit(map: &[u8], index: usize) -> bool {
    map[index].is_ascii_digit()
        && !map
            .get(index.wrapping_sub(1))
            .map_or(false, u8::is_ascii_digit)
}

#[inline(always)]
fn last_digit_index(map: &[u8], first_digit_index: usize) -> usize {
    (first_digit_index + 1..first_digit_index + 4)
        .find(|i| !map[*i].is_ascii_digit())
        .unwrap()
        - 1
}

#[inline(always)]
fn into_number(map: &[u8], first_digit_index: usize, last_digit_index: usize) -> usize {
    atoi::atoi(&map[first_digit_index..last_digit_index + 1]).unwrap()
}

#[inline(always)]
fn bounding_box_offsets(
    map_width: isize,
    num_width: usize,
) -> std::iter::Chain<
    std::iter::Chain<std::ops::Range<isize>, std::array::IntoIter<isize, 2>>,
    std::ops::Range<isize>,
> {
    let width_offset = num_width as isize + 1;
    (-map_width - 2..-map_width + width_offset)
        .chain([-1, width_offset])
        .chain(map_width..map_width + width_offset + 2)
}

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
