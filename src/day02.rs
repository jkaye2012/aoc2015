type Dimensions = (usize, usize, usize);

#[aoc_generator(day2)]
pub fn generate(input: &str) -> Vec<Dimensions> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split('x');
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn total_area(dims: &[Dimensions]) -> usize {
    dims.iter()
        .map(|dim| {
            let (l, w, h) = dim;
            let a = l * w;
            let b = w * h;
            let c = l * h;
            a.min(b).min(c) + 2 * (a + b + c)
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn ribbon(dims: &[Dimensions]) -> usize {
    dims.iter()
        .map(|dim| {
            let (l, w, h) = dim;
            let a = 2 * l + 2 * w;
            let b = 2 * w + 2 * h;
            let c = 2 * l + 2 * h;
            l * w * h + a.min(b).min(c)
        })
        .sum()
}
