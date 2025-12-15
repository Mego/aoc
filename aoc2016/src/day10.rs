use std::cell::RefCell;

use itertools::Itertools;
use rustc_hash::FxHashMap;

struct Bot {
    inv: [Option<u8>; 2],
    lo: (bool, u8),
    hi: (bool, u8),
}

impl Bot {
    fn new(lo: (bool, u8), hi: (bool, u8)) -> Self {
        Self {
            lo,
            hi,
            inv: Default::default(),
        }
    }

    fn take(&mut self, val: u8) {
        if self.inv[0].is_none() {
            self.inv[0] = Some(val);
        } else if self.inv[1].is_none() {
            self.inv[1] = Some(val);
        } else {
            panic!("tried to take when inv is full");
        }
    }

    fn give(
        &mut self,
        bots: &FxHashMap<u8, RefCell<Bot>>,
        outputs: &mut FxHashMap<u8, u8>,
    ) -> bool {
        if let [Some(v1), Some(v2)] = self.inv {
            if self.lo.0 {
                let lo_bot = &bots[&self.lo.1];
                lo_bot.borrow_mut().take(v1.min(v2));
            } else {
                outputs.insert(self.lo.1, v1.min(v2));
            }
            if self.hi.0 {
                let hi_bot = &bots[&self.hi.1];
                hi_bot.borrow_mut().take(v1.max(v2));
            } else {
                outputs.insert(self.hi.1, v1.min(v2));
            }
            self.inv = [None, None];
            true
        } else {
            false
        }
    }
}

pub fn part1(input: &str) -> impl ToString {
    let mut bots = FxHashMap::default();
    let mut outputs = FxHashMap::default();
    let mut directions = vec![];
    const GOAL_LO: u8 = 17;
    const GOAL_HI: u8 = 61;
    input.lines().for_each(|l| {
        let parts = l.split_ascii_whitespace().collect_vec();
        match parts[0] {
            "value" => {
                let val: u8 = parts[1].parse().unwrap();
                let bot: u8 = parts.last().unwrap().parse().unwrap();
                directions.push((val, bot));
            }
            "bot" => {
                let bot: u8 = parts[1].parse().unwrap();
                let lo: u8 = parts[6].parse().unwrap();
                let hi: u8 = parts.last().unwrap().parse().unwrap();
                bots.insert(
                    bot,
                    RefCell::new(Bot::new((parts[5] == "bot", lo), (parts[10] == "bot", hi))),
                );
            }
            _ => unimplemented!(),
        }
    });
    for (val, bot_id) in directions {
        {
            let mut d_bot = bots[&bot_id].borrow_mut();
            d_bot.take(val);
            if let [Some(a), Some(b)] = d_bot.inv
                && a.min(b) == GOAL_LO
                && a.max(b) == GOAL_HI
            {
                return bot_id;
            }
        }
        loop {
            let mut updated = false;

            for (&id, bot) in bots.iter() {
                if let [Some(a), Some(b)] = bot.borrow().inv
                    && a.min(b) == GOAL_LO
                    && a.max(b) == GOAL_HI
                {
                    return id;
                }
                updated |= bot.borrow_mut().give(&bots, &mut outputs);
            }

            if !updated {
                break;
            }
        }
    }

    unreachable!()
}

pub fn part2(input: &str) -> impl ToString {
    let mut bots = FxHashMap::default();
    let mut outputs = FxHashMap::default();
    let mut directions = vec![];
    input.lines().for_each(|l| {
        let parts = l.split_ascii_whitespace().collect_vec();
        match parts[0] {
            "value" => {
                let val: u8 = parts[1].parse().unwrap();
                let bot: u8 = parts.last().unwrap().parse().unwrap();
                directions.push((val, bot));
            }
            "bot" => {
                let bot: u8 = parts[1].parse().unwrap();
                let lo: u8 = parts[6].parse().unwrap();
                let hi: u8 = parts.last().unwrap().parse().unwrap();
                bots.insert(
                    bot,
                    RefCell::new(Bot::new((parts[5] == "bot", lo), (parts[10] == "bot", hi))),
                );
            }
            _ => unimplemented!(),
        }
    });
    for (val, bot_id) in directions {
        {
            let mut d_bot = bots[&bot_id].borrow_mut();
            d_bot.take(val);
        }
        loop {
            let mut updated = false;

            for (_, bot) in bots.iter() {
                updated |= bot.borrow_mut().give(&bots, &mut outputs);
            }

            if !updated {
                break;
            }
        }
    }

    outputs[&0] as u64 * outputs[&1] as u64 * outputs[&2] as u64
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, sync::LazyLock};
    use util::check;

    const YEAR: u16 = 2016;
    const DAY: u8 = 10;
    static INPUT: LazyLock<String> =
        LazyLock::new(|| fs::read_to_string(util::input_path(YEAR, DAY)).unwrap());

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part1() {
        assert!(check(YEAR, DAY, 1, &part1(&INPUT).to_string()).unwrap());
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part2() {
        assert!(check(YEAR, DAY, 2, &part2(&INPUT).to_string()).unwrap());
    }
}
