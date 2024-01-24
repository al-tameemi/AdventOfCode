use std::ops::RangeInclusive;

#[inline(always)]
pub fn is_first_digit(map: &[u8], index: usize) -> bool {
    map[index].is_ascii_digit()
        && !map
            .get(index.wrapping_sub(1))
            .map_or(false, u8::is_ascii_digit)
}

#[inline(always)]
pub fn last_digit_index(map: &[u8], first_digit_index: usize) -> usize {
    (first_digit_index + 1..first_digit_index + 4)
        .find(|i| !map[*i].is_ascii_digit())
        .unwrap()
        - 1
}

#[inline(always)]
pub fn into_number(map: &[u8], first_digit_index: usize, last_digit_index: usize) -> usize {
    atoi::atoi(&map[first_digit_index..last_digit_index + 1]).unwrap()
}

#[inline(always)]
pub fn bounding_box_offsets(
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

pub fn single_bounding_box_offset(map_width: isize) -> std::iter::Chain<std::iter::Chain<RangeInclusive<isize>, std::array::IntoIter<isize, 2>>, RangeInclusive<isize>> {
    (-map_width - 2..=-map_width).chain([-1, 1]).chain(map_width..=map_width + 2)
}