const ARITY: usize = 127;

#[aoc(day3, part1)]
pub fn santa_houses(input: &str) -> usize {
    let mut loc = ARITY * ARITY;
    let mut visits = [false; ARITY * ARITY * 4];
    let mut res = 1;
    visits[loc] = true;
    for dir in input.chars() {
        match dir {
            '^' => loc -= 128,
            '>' => loc += 1,
            'v' => loc += 128,
            '<' => loc -= 1,
            _ => unreachable!(),
        }
        res += !visits[loc] as usize;
        visits[loc] = true;
    }
    res
}

#[aoc(day3, part2)]
pub fn robo_santa(input: &str) -> usize {
    let mut locs = [ARITY * ARITY; 2];
    let mut visits = [false; ARITY * ARITY * 4];
    let mut res = 1;
    visits[ARITY * ARITY] = true;
    for (idx, dir) in input.chars().enumerate() {
        let loc = &mut locs[idx % 2];
        match dir {
            '^' => *loc -= 128,
            '>' => *loc += 1,
            'v' => *loc += 128,
            '<' => *loc -= 1,
            _ => unreachable!(),
        }
        res += !visits[*loc] as usize;
        visits[*loc] = true;
    }
    res
}
