use itertools::Itertools;

#[derive(Debug, Clone, Copy, Default)]
struct Data {
    children: Option<u64>,
    cats: Option<u64>,
    samoyeds: Option<u64>,
    pomeranians: Option<u64>,
    akitas: Option<u64>,
    vizslas: Option<u64>,
    goldfish: Option<u64>,
    trees: Option<u64>,
    cars: Option<u64>,
    perfumes: Option<u64>,
}

impl Data {
    fn day2_match(&self, other: &Self) -> bool {
        (self.children.is_none() || other.children.is_none() || self.children == other.children)
            && (self.cats.is_none() || other.cats.is_none() || self.cats > other.cats)
            && (self.samoyeds.is_none()
                || other.samoyeds.is_none()
                || self.samoyeds == other.samoyeds)
            && (self.pomeranians.is_none()
                || other.pomeranians.is_none()
                || self.pomeranians < other.pomeranians)
            && (self.akitas.is_none() || other.akitas.is_none() || self.akitas == other.akitas)
            && (self.vizslas.is_none() || other.vizslas.is_none() || self.vizslas == other.vizslas)
            && (self.goldfish.is_none()
                || other.goldfish.is_none()
                || self.goldfish < other.goldfish)
            && (self.trees.is_none() || other.trees.is_none() || self.trees > other.trees)
            && (self.cars.is_none() || other.cars.is_none() || self.cars == other.cars)
            && (self.perfumes.is_none()
                || other.perfumes.is_none()
                || self.perfumes == other.perfumes)
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        (self.children.is_none() || other.children.is_none() || self.children == other.children)
            && (self.cats.is_none() || other.cats.is_none() || self.cats == other.cats)
            && (self.samoyeds.is_none()
                || other.samoyeds.is_none()
                || self.samoyeds == other.samoyeds)
            && (self.pomeranians.is_none()
                || other.pomeranians.is_none()
                || self.pomeranians == other.pomeranians)
            && (self.akitas.is_none() || other.akitas.is_none() || self.akitas == other.akitas)
            && (self.vizslas.is_none() || other.vizslas.is_none() || self.vizslas == other.vizslas)
            && (self.goldfish.is_none()
                || other.goldfish.is_none()
                || self.goldfish == other.goldfish)
            && (self.trees.is_none() || other.trees.is_none() || self.trees == other.trees)
            && (self.cars.is_none() || other.cars.is_none() || self.cars == other.cars)
            && (self.perfumes.is_none()
                || other.perfumes.is_none()
                || self.perfumes == other.perfumes)
    }
}

fn parse_aunt(line: &str) -> Data {
    let colon_pos = line.find(':').unwrap();
    let mut aunt = Data::default();
    line[colon_pos + 1..].split(", ").for_each(|d| {
        let kv = d.split(": ").collect_vec();
        let (key, value) = (kv[0].trim(), kv[1].trim().parse::<u64>().unwrap());
        match key {
            "children" => {
                aunt.children = Some(value);
            }
            "cats" => {
                aunt.cats = Some(value);
            }
            "samoyeds" => {
                aunt.samoyeds = Some(value);
            }
            "pomeranians" => {
                aunt.pomeranians = Some(value);
            }
            "akitas" => {
                aunt.akitas = Some(value);
            }
            "vizslas" => {
                aunt.vizslas = Some(value);
            }
            "goldfish" => {
                aunt.goldfish = Some(value);
            }
            "trees" => {
                aunt.trees = Some(value);
            }
            "cars" => {
                aunt.cars = Some(value);
            }
            "perfumes" => {
                aunt.perfumes = Some(value);
            }
            _ => panic!("{key}"),
        }
    });
    aunt
}

impl Eq for Data {}

pub fn part1(input: &str) -> impl ToString {
    let goal = Data {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    let aunts = input.lines().map(parse_aunt).collect_vec();
    aunts.iter().position(|&a| a == goal).unwrap() + 1
}

pub fn part2(input: &str) -> impl ToString {
    let goal = Data {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    let aunts = input.lines().map(parse_aunt).collect_vec();
    aunts.iter().position(|&a| a.day2_match(&goal)).unwrap() + 1
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2015;
    const DAY: u8 = 16;
    static INPUT: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string(util::input_path(YEAR, DAY)).unwrap());

    #[test]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }

    #[test]
    fn test_part2() {
        assert!(check(YEAR, DAY, 2, &part2(&INPUT).to_string()).unwrap());
    }
}
