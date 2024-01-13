use rustc_hash::FxHashMap;

type WireIdx = usize;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    And1(WireIdx),
    And(WireIdx, WireIdx),
    Or(WireIdx, WireIdx),
    Lshift(WireIdx, u16),
    Rshift(WireIdx, u16),
    Not(WireIdx),
    Passthrough(WireIdx),
    Constant(u16),
}

#[derive(Debug, Clone, Copy)]
pub struct Wire {
    op: Op,
    value: Option<u16>,
}

impl Wire {
    pub fn new(op: Op) -> Self {
        Self { op, value: None }
    }
}

#[derive(Debug, Clone)]
pub struct Circuit {
    wires: Vec<Wire>,
    a: WireIdx,
    b: WireIdx,
}

fn wire_idx(
    idxs: &mut FxHashMap<String, WireIdx>,
    wires: &mut Vec<Option<Wire>>,
    name: &str,
) -> WireIdx {
    if !idxs.contains_key(name) {
        idxs.insert(name.to_string(), idxs.len());
        wires.push(None);
    }
    idxs[name]
}

#[aoc_generator(day7)]
pub fn generate(input: &str) -> Circuit {
    let mut idxs = FxHashMap::default();
    let mut wires = Vec::new();
    let mut a = 0;
    let mut b = 0;

    for line in input.lines() {
        let (idx, op, is_a, is_b) = if line.starts_with("1 AND ") {
            let (from, to) = line["1 AND ".len()..].split_once(" -> ").unwrap();
            let from_idx = wire_idx(&mut idxs, &mut wires, from);
            let to_idx = wire_idx(&mut idxs, &mut wires, to);
            (to_idx, Op::And1(from_idx), to == "a", to == "b")
        } else if line.starts_with("NOT ") {
            let (from, to) = line["NOT ".len()..].split_once(" -> ").unwrap();
            let from_idx = wire_idx(&mut idxs, &mut wires, from);
            let to_idx = wire_idx(&mut idxs, &mut wires, to);
            (to_idx, Op::Not(from_idx), to == "a", to == "b")
        } else {
            let (before, to) = line.split_once(" -> ").unwrap();
            let to_idx = wire_idx(&mut idxs, &mut wires, to);
            let is_a = to == "a";
            let is_b = to == "b";
            if let Ok(n) = before.parse::<u16>() {
                (to_idx, Op::Constant(n), is_a, is_b)
            } else {
                let mut it = before.split(' ');
                let a = it.next().unwrap();
                let a_idx = wire_idx(&mut idxs, &mut wires, a);
                if let Some(o) = it.next() {
                    let b = it.next().unwrap();
                    match o {
                        "AND" => {
                            let b_idx = wire_idx(&mut idxs, &mut wires, b);
                            (to_idx, Op::And(a_idx, b_idx), is_a, is_b)
                        }
                        "OR" => {
                            let b_idx = wire_idx(&mut idxs, &mut wires, b);
                            (to_idx, Op::Or(a_idx, b_idx), is_a, is_b)
                        }
                        "LSHIFT" => (to_idx, Op::Lshift(a_idx, b.parse().unwrap()), is_a, is_b),
                        "RSHIFT" => (to_idx, Op::Rshift(a_idx, b.parse().unwrap()), is_a, is_b),
                        _ => unreachable!(),
                    }
                } else {
                    (to_idx, Op::Passthrough(a_idx), is_a, is_b)
                }
            }
        };

        wires[idx] = Some(Wire::new(op));
        if is_a {
            a = idx;
        }
        if is_b {
            b = idx;
        }
    }

    Circuit {
        wires: wires.iter().map(|w| w.unwrap()).collect(),
        a,
        b,
    }
}

fn propagate(wires: &mut [Wire], idx: WireIdx) -> u16 {
    if let Some(val) = wires[idx].value {
        return val;
    }

    let op = wires[idx].op;
    let val = match op {
        Op::And1(a) => propagate(wires, a) & 1,
        Op::And(a, b) => propagate(wires, a) & propagate(wires, b),
        Op::Or(a, b) => propagate(wires, a) | propagate(wires, b),
        Op::Lshift(a, n) => propagate(wires, a) << n,
        Op::Rshift(a, n) => propagate(wires, a) >> n,
        Op::Not(a) => !propagate(wires, a),
        Op::Passthrough(a) => propagate(wires, a),
        Op::Constant(v) => v,
    };

    wires[idx].value = Some(val);
    val
}

#[aoc(day7, part1)]
pub fn wire_a(circuit: &Circuit) -> u16 {
    propagate(&mut circuit.wires.clone(), circuit.a)
}

#[aoc(day7, part2)]
pub fn wire_b(circuit: &Circuit) -> u16 {
    let mut wires = circuit.wires.clone();
    wires[circuit.b] = Wire::new(Op::Constant(46065));
    propagate(&mut wires, circuit.a)
}
