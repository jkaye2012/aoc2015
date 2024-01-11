#[derive(Debug)]
pub enum Action {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
pub struct Instruction {
    min_y: usize,
    min_x: usize,
    max_y: usize,
    max_x: usize,
    action: Action,
}

#[aoc_generator(day6)]
pub fn generate(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut it = l.split(' ');
            let action = if it.next().unwrap() == "toggle" {
                Action::Toggle
            } else {
                if it.next().unwrap() == "on" {
                    Action::On
                } else {
                    Action::Off
                }
            };
            let (min_y, min_x) = it.next().unwrap().split_once(',').unwrap();
            it.next();
            let (max_y, max_x) = it.next().unwrap().split_once(',').unwrap();

            Instruction {
                min_y: min_y.parse().unwrap(),
                min_x: min_x.parse().unwrap(),
                max_y: max_y.parse().unwrap(),
                max_x: max_x.parse().unwrap(),
                action,
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn lights_on(instructions: &[Instruction]) -> usize {
    let mut lights = [[false; 1000]; 1000];
    for instruction in instructions.iter() {
        for row in &mut lights[instruction.min_y..=instruction.max_y] {
            let cols = &mut row[instruction.min_x..=instruction.max_x];
            match instruction.action {
                Action::On => cols.fill(true),
                Action::Off => cols.fill(false),
                Action::Toggle => cols.iter_mut().for_each(|c| *c = !*c),
            }
        }
    }

    lights
        .iter()
        .map(|row| row.iter().filter(|l| **l).count())
        .sum()
}

#[aoc(day6, part2)]
pub fn total_brightness(instructions: &[Instruction]) -> usize {
    let mut lights = [[0usize; 1000]; 1000];
    for instruction in instructions.iter() {
        for row in &mut lights[instruction.min_y..=instruction.max_y] {
            let cols = &mut row[instruction.min_x..=instruction.max_x];
            match instruction.action {
                Action::On => cols.iter_mut().for_each(|c| *c += 1),
                Action::Off => cols.iter_mut().for_each(|c| *c = c.saturating_sub(1)),
                Action::Toggle => cols.iter_mut().for_each(|c| *c += 2),
            }
        }
    }

    lights.iter().map(|row| row.iter().sum::<usize>()).sum()
}
