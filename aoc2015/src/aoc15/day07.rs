use std::collections::{HashMap, hash_map::Entry};

use itertools::Itertools;

#[derive(Clone, Copy)]
enum InputValue<'a> {
    Raw(u16),
    Wire(&'a str),
}

#[derive(Clone, Copy)]
enum Gate<'a> {
    Const(u16),
    And(InputValue<'a>, InputValue<'a>),
    Or(InputValue<'a>, InputValue<'a>),
    Lshift(InputValue<'a>, u16),
    Rshift(InputValue<'a>, u16),
    Not(InputValue<'a>),
    Direct(InputValue<'a>),
}

impl<'a> Gate<'a> {
    const fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    const fn value(&self) -> Option<u16> {
        match self {
            Self::Const(a) => Some(*a),
            _ => None,
        }
    }
}

#[derive(Default, Clone)]
struct Circuit<'a>(HashMap<&'a str, Gate<'a>>);

impl<'a> Circuit<'a> {
    fn add_gate(&mut self, output: &'a str, gate: Gate<'a>) -> Result<(), &'a str> {
        if let Entry::Vacant(e) = self.0.entry(output) {
            e.insert(gate);
            Ok(())
        } else {
            Err(output)
        }
    }

    fn resolve(&self) -> Self {
        let mut resolved = self.clone();
        while !resolved.0["a"].is_const() {
            let to_resolve = resolved
                .0
                .iter()
                .filter_map(|(&k, &v)| (!v.is_const()).then_some((k, v)))
                .collect_vec();
            for (k, v) in to_resolve {
                match v {
                    Gate::Direct(a) => {
                        let a = match a {
                            InputValue::Raw(a) => Some(a),
                            InputValue::Wire(a) => {
                                if let Gate::Const(a) = resolved.0[a] {
                                    Some(a)
                                } else {
                                    None
                                }
                            }
                        };
                        if let Some(a) = a {
                            *resolved.0.get_mut(k).unwrap() = Gate::Const(a);
                        }
                    }
                    Gate::And(a, b) => {
                        let a = match a {
                            InputValue::Raw(a) => Some(a),
                            InputValue::Wire(a) => {
                                if let Gate::Const(a) = resolved.0[a] {
                                    Some(a)
                                } else {
                                    None
                                }
                            }
                        };
                        let b = match b {
                            InputValue::Raw(b) => Some(b),
                            InputValue::Wire(b) => {
                                if let Gate::Const(b) = resolved.0[b] {
                                    Some(b)
                                } else {
                                    None
                                }
                            }
                        };
                        if let Some(a) = a
                            && let Some(b) = b
                        {
                            *resolved.0.get_mut(k).unwrap() = Gate::Const(a & b);
                        }
                    }
                    Gate::Or(a, b) => {
                        let a = match a {
                            InputValue::Raw(a) => Some(a),
                            InputValue::Wire(a) => {
                                if let Gate::Const(a) = resolved.0[a] {
                                    Some(a)
                                } else {
                                    None
                                }
                            }
                        };
                        let b = match b {
                            InputValue::Raw(b) => Some(b),
                            InputValue::Wire(b) => {
                                if let Gate::Const(b) = resolved.0[b] {
                                    Some(b)
                                } else {
                                    None
                                }
                            }
                        };
                        if let Some(a) = a
                            && let Some(b) = b
                        {
                            *resolved.0.get_mut(k).unwrap() = Gate::Const(a | b);
                        }
                    }
                    Gate::Lshift(a, b) => {
                        let a = match a {
                            InputValue::Raw(a) => Some(a),
                            InputValue::Wire(a) => {
                                if let Gate::Const(a) = resolved.0[a] {
                                    Some(a)
                                } else {
                                    None
                                }
                            }
                        };
                        if let Some(a) = a {
                            *resolved.0.get_mut(k).unwrap() = Gate::Const(a << b);
                        }
                    }
                    Gate::Rshift(a, b) => {
                        let a = match a {
                            InputValue::Raw(a) => Some(a),
                            InputValue::Wire(a) => {
                                if let Gate::Const(a) = resolved.0[a] {
                                    Some(a)
                                } else {
                                    None
                                }
                            }
                        };
                        if let Some(a) = a {
                            *resolved.0.get_mut(k).unwrap() = Gate::Const(a >> b);
                        }
                    }
                    Gate::Not(a) => {
                        let a = match a {
                            InputValue::Raw(a) => Some(a),
                            InputValue::Wire(a) => {
                                if let Gate::Const(a) = resolved.0[a] {
                                    Some(a)
                                } else {
                                    None
                                }
                            }
                        };
                        if let Some(a) = a {
                            *resolved.0.get_mut(k).unwrap() = Gate::Const(!a);
                        }
                    }
                    Gate::Const(_) => unreachable!(),
                }
            }
        }
        resolved
    }
}

peg::parser! {
    grammar circuit_parser() for str {
        rule number() -> u16
            = n:$(['0'..='9']+) {? n.parse().or(Err("u16")) }

        rule wire() -> InputValue<'input>
            = s:$(['a'..='z']+) { InputValue::Wire(s) }

        rule raw() -> InputValue<'input>
            = n:number() { InputValue::Raw(n) }

        rule input() -> InputValue<'input>
            = wire() / raw()

        rule const() -> Gate<'input>
            = n:number() { Gate::Const(n) }

        rule and() -> Gate<'input>
            = a:input() " AND " b:input() { Gate::And(a, b) }

        rule or() -> Gate<'input>
            = a:input() " OR " b:input() { Gate::Or(a, b) }

        rule lshift() -> Gate<'input>
            = a:input() " LSHIFT " b:number() { Gate::Lshift(a, b) }

        rule rshift() -> Gate<'input>
            = a:input() " RSHIFT " b:number() { Gate::Rshift(a, b) }

        rule not() -> Gate<'input>
            = "NOT " a:input() { Gate::Not(a) }

        rule direct() -> Gate<'input>
            = a:wire() { Gate::Direct(a) }

        rule gate() -> (Gate<'input>, &'input str)
            = g:(and() / or() / lshift() / rshift() / not() / direct() / const()) " -> " w:$(wire()) { (g, w) }

        pub rule circuit() -> Circuit<'input>
            = x:(g:gate() ++ "\n") {?
                let mut circuit = Circuit::default();
                for gate in x {
                    circuit.add_gate(gate.1, gate.0).or(Err("duplicate gate output"));
                }
                Ok(circuit)
            }

    }
}

pub fn part1(input: String) -> u64 {
    let circuit = circuit_parser::circuit(&input).unwrap();
    circuit.resolve().0["a"].value().unwrap() as u64
}
pub fn part2(input: String) -> u64 {
    let mut circuit = circuit_parser::circuit(&input).unwrap();
    let resolved = circuit.resolve();
    let a = resolved.0["a"].value().unwrap();
    circuit.0.insert("b", Gate::Const(a));
    circuit.resolve().0["a"].value().unwrap() as u64
}
