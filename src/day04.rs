use md5;

#[aoc(day4, part1)]
pub fn find_hash(input: &str) -> usize {
    let mut res = 1;
    while {
        let mut candidate = res.to_string();
        candidate.insert_str(0, input);
        let hash = md5::compute(candidate.as_bytes());
        !(hash.starts_with(&[0; 2]) && hash.0[2] < 0xF)
    } {
        res += 1;
    }
    res
}

#[aoc(day4, part2)]
pub fn find_hash_6(input: &str) -> usize {
    let mut res = 1;
    while {
        let mut candidate = res.to_string();
        candidate.insert_str(0, input);
        let hash = md5::compute(candidate.as_bytes());
        !hash.starts_with(&[0; 3])
    } {
        res += 1;
    }
    res
}
