use rustc_hash::FxHashMap;

fn is_nice(s: &[u8]) -> bool {
    let mut vowels = 0;
    let mut doubles = 0;
    let mut violations = 0;

    for idx in 0..s.len() - 1 {
        let pair = &s[idx..=idx + 1];
        let fst = pair[0];
        vowels +=
            (fst == b'a' || fst == b'e' || fst == b'i' || fst == b'o' || fst == b'u') as usize;
        doubles += (pair[0] == pair[1]) as usize;
        violations += (pair == b"ab" || pair == b"cd" || pair == b"pq" || pair == b"xy") as usize;
    }
    let last = *s.last().unwrap();
    vowels +=
        (last == b'a' || last == b'e' || last == b'i' || last == b'o' || last == b'u') as usize;

    violations == 0 && vowels >= 3 && doubles >= 1
}

fn is_nicer(s: &[u8]) -> bool {
    let mut pairs = 0;
    let mut pair_map = FxHashMap::default();
    for idx in 0..s.len() - 1 {
        let pair = &s[idx..=idx + 1];
        if let Some(prev) = pair_map.get(&pair) {
            if *prev != idx - 1 {
                pairs += 1;
            }
        } else {
            pair_map.insert(pair, idx);
        }
    }

    let mut repeats = 0;
    for idx in 0..s.len() - 2 {
        repeats += (s[idx] == s[idx + 2]) as usize;
    }

    pairs > 0 && repeats > 0
}

#[aoc(day5, part1)]
pub fn nice_strings(input: &str) -> usize {
    input.lines().map(|l| is_nice(l.as_bytes()) as usize).sum()
}

#[aoc(day5, part2)]
pub fn nicer_strings(input: &str) -> usize {
    input.lines().map(|l| is_nicer(l.as_bytes()) as usize).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nice() {
        assert!(is_nice(b"ugknbfddgicrmopn"));
        assert!(is_nice(b"aaa"));
        assert!(is_nice(b"aeiouu"));
    }

    #[test]
    fn test_naughty() {
        assert!(!is_nice(b"jchzalrnumimnmhp"));
        assert!(!is_nice(b"haegwjzuvuyypxyu"));
        assert!(!is_nice(b"dvszwmarrgswjxmb"));
    }
}
