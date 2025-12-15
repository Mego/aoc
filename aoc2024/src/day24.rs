use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Instruction {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl Instruction {
    fn is_and(&self) -> bool {
        matches!(self, Self::And(_, _))
    }

    fn is_or(&self) -> bool {
        matches!(self, Self::Or(_, _))
    }

    fn is_xor(&self) -> bool {
        matches!(self, Self::Xor(_, _))
    }

    fn operands(&self) -> (&String, &String) {
        match self {
            Self::And(a, b) | Self::Or(a, b) | Self::Xor(a, b) => (a, b),
        }
    }

    fn try_resolve(&self, wires: &HashMap<String, Option<bool>>) -> Option<bool> {
        match self {
            Self::And(a, b) => {
                if let Some(a_val) = wires[a]
                    && let Some(b_val) = wires[b]
                {
                    let val = a_val && b_val;
                    return Some(val);
                }
                None
            }
            Self::Or(a, b) => {
                if let Some(a_val) = wires[a]
                    && let Some(b_val) = wires[b]
                {
                    let val = a_val || b_val;
                    return Some(val);
                }
                None
            }
            Self::Xor(a, b) => {
                if let Some(a_val) = wires[a]
                    && let Some(b_val) = wires[b]
                {
                    let val = a_val ^ b_val;
                    return Some(val);
                }
                None
            }
        }
    }
}

fn resolve(values: &mut HashMap<String, Option<bool>>, cnxs: &HashMap<String, Instruction>) {
    loop {
        let mut updates = HashMap::new();
        for wire in values.keys().by_ref() {
            let value = values[wire];
            if value.is_none()
                && let Some(val) = cnxs[wire].try_resolve(values)
            {
                updates.insert(wire.to_owned(), val);
            }
        }
        if updates.is_empty() {
            break;
        }
        for (wire, val) in updates {
            values.insert(wire, Some(val));
        }
    }
}

fn parse(input: &str) -> (HashMap<String, Option<bool>>, HashMap<String, Instruction>) {
    // :wires:
    let mut wires = HashMap::new();
    let mut res = HashMap::new();
    let (wires_list, ops) = input.split("\n\n").collect_tuple().unwrap();
    for wire in wires_list.lines() {
        let (name, val) = wire.split(": ").collect_tuple().unwrap();
        wires.insert(name.to_owned(), Some(val == "1"));
    }

    for op_line in ops.lines() {
        let (op_str, to) = op_line.split(" -> ").collect_tuple().unwrap();
        let (a_name, op_name, b_name) = op_str.split(" ").collect_tuple().unwrap();
        wires.entry(a_name.to_owned()).or_insert(None);
        wires.entry(b_name.to_owned()).or_insert(None);
        let instr = match op_name {
            "AND" => Instruction::And(a_name.to_owned(), b_name.to_owned()),
            "OR" => Instruction::Or(a_name.to_owned(), b_name.to_owned()),
            "XOR" => Instruction::Xor(a_name.to_owned(), b_name.to_owned()),
            _ => unimplemented!(),
        };
        wires.entry(to.to_owned()).or_insert(None);
        res.insert(to.to_owned(), instr);
    }

    (wires, res)
}

pub fn part1(input: &str) -> impl ToString {
    let (mut wires, cnxs) = parse(input);
    resolve(&mut wires, &cnxs);
    wires
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by_cached_key(|(k, _)| k.clone())
        .rev()
        .map(|(_, v)| v.unwrap())
        .fold(0, |a, v| (a << 1) + if v { 1 } else { Default::default() })
}

fn validate(k: &String, i: &Instruction, cnxs: &HashMap<String, Instruction>) -> bool {
    if k.starts_with("z") && k != "z45" {
        return i.is_xor();
    }

    let (a, b) = i.operands();
    if !(k.starts_with("z")
        || a.starts_with("x") && b.starts_with("y")
        || a.starts_with("y") && b.starts_with("x"))
    {
        return i.is_and() || i.is_or();
    }

    if ((a.starts_with("x") && b.starts_with("y") && a != "x00" && b != "y00")
        || (a.starts_with("y") && b.starts_with("x") && a != "y00" && b != "x00"))
        && i.is_xor()
    {
        return cnxs.iter().any(|(_, i2)| {
            let (a2, b2) = i2.operands();
            (a2 == k || b2 == k) && i2.is_xor()
        });
    }

    if ((a.starts_with("x") && b.starts_with("y") && a != "x00" && b != "y00")
        || (a.starts_with("y") && b.starts_with("x") && a != "y00" && b != "x00"))
        && i.is_and()
    {
        return cnxs.iter().any(|(_, i2)| {
            let (a2, b2) = i2.operands();
            (a2 == k || b2 == k) && i2.is_or()
        });
    }

    true
}

pub fn part2(input: &str) -> impl ToString {
    let (_, cnxs) = parse(input);

    cnxs.iter()
        .filter(|&(k, i)| !validate(k, i, &cnxs))
        .map(|(k, _)| k.clone())
        .sorted()
        .join(",")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2024;
    const DAY: u8 = 24;
    static INPUT: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string(util::input_path(YEAR, DAY)).unwrap());

    #[test]
    #[ignore = "still debugging this one"]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }

    #[test]
    fn test_part2() {
        assert!(check(YEAR, DAY, 2, &part2(&INPUT).to_string()).unwrap());
    }
}
