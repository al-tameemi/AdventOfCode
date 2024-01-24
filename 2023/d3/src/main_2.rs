use d3::*;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let map = include_bytes!("../input");
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;
    let mut numbers = Vec::new();

    let total = (0..map.len())
        .filter(|index| map[*index] == b'*')
        .filter_map(|index| {
            numbers.clear();
            numbers.extend(
                single_bounding_box_offset(width)
                    .map(|position| (index as isize + position) as usize)
                    .filter(|position| map[*position].is_ascii_digit())
                    .flat_map(|position| {
                        (position.saturating_sub(2)..=position)
                            .rev()
                            .take_while(|index| map[*index].is_ascii_digit())
                            .last()
                    }),
            );
            numbers.dedup();
            (numbers.len() == 2).then(|| {
                numbers
                    .iter()
                    .map(|index| into_number(map, *index, index + 3))
                    .product::<usize>()
            })
        })
        .sum::<usize>();

    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("{}", total);
}
