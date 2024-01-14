#[aoc(day8, part1)]
pub fn memory_diff(input: &str) -> usize {
    let mut diff = 2;
    let mut idx = 0;
    let bytes = input.as_bytes();
    while idx < bytes.len() - 1 {
        if bytes[idx] == b'\n' {
            diff += 2;
        } else if bytes[idx] == b'\\' {
            let d = bytes[idx + 1] / b'x' * 2 + 1;
            diff += d as usize;
            idx += d as usize;
        }
        idx += 1;
    }

    diff
}

#[aoc(day8, part2)]
pub fn encoded_diff(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.chars().filter(|c| *c == '\\').count() + l.chars().filter(|c| *c == '"').count() + 2
        })
        .sum()
}
