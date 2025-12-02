use std::{cell::RefCell, convert::Infallible, rc::Rc, str::FromStr};

use exhaust::Exhaust;
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    hp: u8,
    mana: u16,
    mana_used: u16,
    boss_hp: u8,
    boss_damage: u8,
    shield_timer: u8,
    poison_timer: u8,
    recharge_timer: u8,
}

#[derive(Debug, Clone, Copy, Exhaust)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> u16 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }
}

impl State {
    fn can_use_spell(&self, spell: Spell) -> bool {
        self.mana >= spell.cost()
            && match spell {
                Spell::Shield => self.shield_timer == 0,
                Spell::Poison => self.poison_timer == 0,
                Spell::Recharge => self.recharge_timer == 0,
                _ => true,
            }
    }

    fn do_turn(mut self, spell: Spell) -> Option<Self> {
        // player turn
        if self.poison_timer > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.mana += 101;
            self.recharge_timer -= 1;
        }
        self.shield_timer = self.shield_timer.saturating_sub(1);

        if self.boss_hp == 0 {
            return Some(self);
        }

        if !self.can_use_spell(spell) {
            return None;
        }
        match spell {
            Spell::MagicMissile => {
                self.boss_hp = self.boss_hp.saturating_sub(4);
            }
            Spell::Drain => {
                self.boss_hp = self.boss_hp.saturating_sub(2);
                self.hp += 2;
            }
            Spell::Shield => {
                self.shield_timer = 6;
            }
            Spell::Poison => {
                self.poison_timer = 6;
            }
            Spell::Recharge => {
                self.recharge_timer = 5;
            }
        };
        self.mana -= spell.cost();
        self.mana_used += spell.cost();

        if self.boss_hp == 0 {
            return Some(self);
        }

        // boss turn
        if self.poison_timer > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.mana += 101;
            self.recharge_timer -= 1;
        }

        self.hp = self.hp.saturating_sub(
            self.boss_damage
                .saturating_sub(if self.shield_timer > 0 { 7 } else { 0 })
                .max(1),
        );
        self.shield_timer = self.shield_timer.saturating_sub(1);

        Some(self)
    }

    fn do_turn2(mut self, spell: Spell) -> Option<Self> {
        // player turn
        self.hp -= 1;
        if self.hp == 0 {
            return Some(self);
        }
        if self.poison_timer > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.mana += 101;
            self.recharge_timer -= 1;
        }
        self.shield_timer = self.shield_timer.saturating_sub(1);

        if self.boss_hp == 0 {
            return Some(self);
        }

        if !self.can_use_spell(spell) {
            return None;
        }
        match spell {
            Spell::MagicMissile => {
                self.boss_hp = self.boss_hp.saturating_sub(4);
            }
            Spell::Drain => {
                self.boss_hp = self.boss_hp.saturating_sub(2);
                self.hp += 2;
            }
            Spell::Shield => {
                self.shield_timer = 6;
            }
            Spell::Poison => {
                self.poison_timer = 6;
            }
            Spell::Recharge => {
                self.recharge_timer = 5;
            }
        };
        self.mana -= spell.cost();
        self.mana_used += spell.cost();

        if self.boss_hp == 0 {
            return Some(self);
        }

        // boss turn
        if self.poison_timer > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.mana += 101;
            self.recharge_timer -= 1;
        }

        self.hp = self.hp.saturating_sub(
            self.boss_damage
                .saturating_sub(if self.shield_timer > 0 { 7 } else { 0 })
                .max(1),
        );
        self.shield_timer = self.shield_timer.saturating_sub(1);

        Some(self)
    }
}

impl FromStr for State {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (boss_hp, boss_damage) = s
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u8>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Ok(Self {
            hp: 50,
            mana: 500,
            mana_used: 0,
            boss_hp,
            boss_damage,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
        })
    }
}

pub fn part1(input: String) -> u16 {
    let init: State = input.parse().unwrap();
    let best = Rc::new(RefCell::new(u16::MAX));
    let mut states = FxHashSet::default();
    states.insert(init);
    while !states.is_empty() {
        states = states
            .into_iter()
            .flat_map(|s| {
                Spell::exhaust().filter_map({
                    let value = best.clone();
                    move |sp| {
                        if let Some(next) = s.do_turn(sp)
                            && next.hp > 0
                            && next.mana_used <= *value.borrow()
                        {
                            if next.boss_hp == 0 {
                                *value.borrow_mut() = next.mana_used;
                                None
                            } else {
                                Some(next)
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .collect();
    }
    Rc::into_inner(best).unwrap().into_inner()
}
pub fn part2(input: String) -> u16 {
    let init: State = input.parse().unwrap();
    let best = Rc::new(RefCell::new(u16::MAX));
    let mut states = FxHashSet::default();
    states.insert(init);
    while !states.is_empty() {
        states = states
            .into_iter()
            .flat_map(|s| {
                Spell::exhaust().filter_map({
                    let value = best.clone();
                    move |sp| {
                        if let Some(next) = s.do_turn2(sp)
                            && next.hp > 0
                            && next.mana_used <= *value.borrow()
                        {
                            if next.boss_hp == 0 {
                                *value.borrow_mut() = next.mana_used;
                                None
                            } else {
                                Some(next)
                            }
                        } else {
                            None
                        }
                    }
                })
            })
            .collect();
    }
    Rc::into_inner(best).unwrap().into_inner()
}
