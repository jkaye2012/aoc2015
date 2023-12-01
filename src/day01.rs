#[aoc(day1, part1)]
pub fn find_floor(input: &str) -> isize {
    input
        .as_bytes()
        .iter()
        .map(|b| (b - b'(') as isize * -2 + 1)
        .sum()
}

#[aoc(day1, part2)]
pub fn find_basement(input: &str) -> usize {
    input
        .as_bytes()
        .iter()
        .scan(0, |acc, elem| {
            *acc += (elem - b'(') as isize * -2 + 1;
            Some(*acc)
        })
        .enumerate()
        .find(|(_, f)| *f < 0)
        .unwrap()
        .0
        + 1
}
