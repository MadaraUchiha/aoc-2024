use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::solution::Solution;

pub struct Day24;

impl Solution for Day24 {
    type Answer = String;
    fn day(&self) -> u8 {
        24
    }

    fn part1(input: &str) -> Result<Self::Answer> {
        let mut circuit = input.parse::<Circuit>()?;
        circuit.resolve_values();
        Ok(circuit.read_z_value().to_string())
    }

    fn part2(input: &str) -> Result<Self::Answer> {
        let circuit = input.parse::<Circuit>()?;
        let bit_length = (circuit.values.len() / 2) as u8;
        let mut swapped = HashSet::new();

        let z00 = circuit
            .gates
            .values()
            .find(|gate| {
                if let Gate::Xor(_, _, _) = gate {
                    if gate.is_input("x00") && gate.is_input("y00") {
                        return true;
                    }
                }
                false
            })
            .unwrap();

        if z00.output() != "z00" {
            swapped.insert(z00.output().clone());
        }

        let mut carry = circuit
            .gates
            .values()
            .find_map(|gate| {
                if let Gate::And(_, _, _) = gate {
                    if gate.is_input("x00") && gate.is_input("y00") {
                        return Some(gate.output().clone());
                    }
                }
                None
            })
            .unwrap();

        for bit in 1..bit_length {
            let x = format!("x{:02}", bit);
            let y = format!("y{:02}", bit);
            let z = format!("z{:02}", bit);

            let basic_add = circuit
                .gates
                .values()
                .find(|&gate| {
                    if let Gate::Xor(_, _, _) = gate {
                        if gate.is_input(&x) && gate.is_input(&y) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap()
                .output();

            let add = circuit
                .gates
                .values()
                .find(|&gate| {
                    if let Gate::Xor(_, _, _) = gate {
                        if gate.is_input(&carry) || gate.is_input(&basic_add) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap();

            if add.output() != z {
                swapped.insert(z);
                swapped.insert(add.output());
            }

            if !add.is_input(&basic_add) {
                swapped.insert(basic_add.clone());
            }

            if !add.is_input(&carry) {
                swapped.insert(carry.clone());
            }

            let basic_carry = circuit
                .gates
                .values()
                .find(|&gate| {
                    if let Gate::And(_, _, _) = gate {
                        if gate.is_input(&x) && gate.is_input(&y) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap()
                .output();

            let cascade_carry = circuit
                .gates
                .values()
                .find(|&gate| {
                    if let Gate::And(_, _, _) = gate {
                        if gate.is_input(&carry) || gate.is_input(&basic_add) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap();

            if !cascade_carry.is_input(&basic_add) {
                swapped.insert(basic_add.clone());
            }

            if !cascade_carry.is_input(&carry) {
                swapped.insert(carry.clone());
            }

            let carry_gate = circuit
                .gates
                .values()
                .find(|&gate| {
                    if let Gate::Or(_, _, _) = gate {
                        if gate.is_input(&basic_carry) || gate.is_input(&cascade_carry.output()) {
                            return true;
                        }
                    }
                    false
                })
                .unwrap();

            if !carry_gate.is_input(&basic_carry) {
                swapped.insert(basic_carry);
            }

            if !carry_gate.is_input(&cascade_carry.output()) {
                swapped.insert(cascade_carry.output());
            }

            carry = carry_gate.output();
        }

        let mut swapped = swapped.iter().collect_vec();
        swapped.sort();
        Ok(swapped.iter().join(","))
    }
}

enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

struct Circuit {
    values: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

impl Circuit {
    fn resolve_values(&mut self) {
        let mut unresolved = HashSet::new();
        unresolved.extend(self.gates.keys());
        for key in self.values.keys() {
            unresolved.remove(key);
        }

        while !unresolved.is_empty() {
            let mut resolved = HashSet::new();
            for &key in unresolved.iter() {
                let gate = self.gates.get(key).unwrap();
                let input1 = &gate.in1();
                let input2 = &gate.in2();
                let output = &gate.output();
                match (self.values.get(input1), self.values.get(input2)) {
                    (Some(&in1_value), Some(&in2_value)) => {
                        let out_value = match gate {
                            Gate::And(_, _, _) => in1_value && in2_value,
                            Gate::Or(_, _, _) => in1_value || in2_value,
                            Gate::Xor(_, _, _) => in1_value ^ in2_value,
                        };
                        self.values.insert(output.clone(), out_value);
                        resolved.insert(key.clone());
                    }
                    _ => continue,
                }
            }

            for key in resolved {
                unresolved.remove(&key);
            }
        }
    }

    fn read_z_value(&self) -> u64 {
        let mut z_value = 0;
        for (key, value) in self.values.iter().filter(|(key, _)| key.starts_with("z")) {
            let index = key[1..].parse::<u64>().unwrap();
            if *value {
                z_value |= 1 << index;
            }
        }
        z_value
    }
}

impl FromStr for Circuit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (values_str, gates_str) = s.split_once("\n\n").ok_or(anyhow!("Invalid input"))?;
        let values = values_str
            .lines()
            .map(|line| {
                let (key, value) = line.split_once(": ").ok_or(anyhow!("Invalid input"))?;
                let val = match value {
                    "1" => true,
                    "0" => false,
                    _ => return Err(anyhow!("Invalid input")),
                };

                Ok((key.to_string(), val))
            })
            .collect::<Result<_>>()?;

        let gates = gates_str
            .lines()
            .map(|line| line.parse::<Gate>())
            .map(|gate| gate.map(|gate| (gate.output(), gate)))
            .collect::<Result<_>>()?;

        Ok(Circuit { values, gates })
    }
}

impl FromStr for Gate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let input1 = String::from(parts[0]);
        let input2 = String::from(parts[2]);
        let output = String::from(parts[4]);

        match parts[1] {
            "AND" => Ok(Gate::And(input1, input2, output)),
            "OR" => Ok(Gate::Or(input1, input2, output)),
            "XOR" => Ok(Gate::Xor(input1, input2, output)),
            _ => Err(anyhow!("Invalid gate")),
        }
    }
}

impl Gate {
    fn output(&self) -> String {
        match self {
            Gate::And(_, _, output) => output.clone(),
            Gate::Or(_, _, output) => output.clone(),
            Gate::Xor(_, _, output) => output.clone(),
        }
    }

    fn in1(&self) -> String {
        match self {
            Gate::And(input1, _, _) => input1.clone(),
            Gate::Or(input1, _, _) => input1.clone(),
            Gate::Xor(input1, _, _) => input1.clone(),
        }
    }

    fn in2(&self) -> String {
        match self {
            Gate::And(_, input2, _) => input2.clone(),
            Gate::Or(_, input2, _) => input2.clone(),
            Gate::Xor(_, input2, _) => input2.clone(),
        }
    }

    fn is_input(&self, input: &str) -> bool {
        let input = input.to_string();
        match self {
            Gate::And(in1, in2, _) => *in1 == input || *in2 == input,
            Gate::Or(in1, in2, _) => *in1 == input || *in2 == input,
            Gate::Xor(in1, in2, _) => *in1 == input || *in2 == input,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day24.run_test1(), "2024");
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day24.run_test2(), "0");
    }
}
