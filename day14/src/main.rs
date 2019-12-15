extern crate regex;

use regex::Regex;
use std::collections::HashMap;

const INPUT: &str = include_str!("../day14.txt");
const PT2_AVAIL_ORE: u64 = 1000000000000;

fn main() {
    println!("Day 14-1: {}", part1(INPUT));
    println!("Day 14-2: {}", part2(INPUT));
}

fn part1(input: &str) -> u64 {
    let (_, fuel) = parse_input(input).calc_multipliers_and_ore_for_one_fuel();
    fuel
}

fn part2(input: &str) -> u64 {
    parse_input(input).calc_fuel_produced_by_avail_ore(PT2_AVAIL_ORE)
}

#[derive(Debug)]
struct SystemOfEquations {
    equations_by_output: HashMap<String, Equation>,
}

impl SystemOfEquations {
    fn add_eq(&mut self, equation: Equation) {
        self.equations_by_output
            .insert(equation.output.id.clone(), equation);
    }

    fn calc_multipliers_and_ore_for_one_fuel(&self) -> (HashMap<String, u64>, u64) {
        //2 AB, 3 A => 1 FUEL
        let mut available: HashMap<String, u64> = HashMap::new();
        let mut mult_counts: HashMap<String, u64> = HashMap::new();
        let mut ore_used: u64 = 0;

        let fuel_eq = self.equations_by_output.get("FUEL").unwrap();
        self.calc_ore_inner(
            fuel_eq.output.id.clone(),
            fuel_eq.output.quantity,
            &mut available,
            &mut mult_counts,
            &mut ore_used,
        );
        (mult_counts, ore_used)
    }

    fn calc_fuel_produced_by_avail_ore(&mut self, ore_avail: u64) -> u64 {
        // calculate the fuel like in part 1 to get the multipliers needed to produce 1 FUEL
        let (mult_counts, ore_per_fuel) = self.calc_multipliers_and_ore_for_one_fuel();

        // modify the system with those multipliers
        for (output_id, count) in mult_counts.iter() {
            self.equations_by_output
                .entry(output_id.to_string())
                .and_modify(|e| e.multiplier = *count);
        }

        let mut available: HashMap<String, u64> = HashMap::new();
        let mut _multiplier_counts = &mut HashMap::new();
        let mut ore_used = 0;

        // can produce at least fuel = the amount of ore divided by the ore needed per fuel, but
        //   more than that due to partial consumption of inputs
        let at_least_fuel = ore_avail / ore_per_fuel;
        let fuel_eq = self.equations_by_output.get("FUEL").unwrap();
        self.calc_ore_inner(
            fuel_eq.output.id.clone(),
            at_least_fuel,
            &mut available,
            &mut HashMap::new(),
            &mut ore_used,
        );

        // brute force from there: produce 1 more fuel until ore is exhausted
        // TODO: Do this in a way that doesn't take minutes with level 3 optimization enabled...
        let mut fuel: u64 = at_least_fuel;
        while ore_used <= ore_avail {
            self.calc_ore_inner(
                fuel_eq.output.id.clone(),
                1,
                &mut available,
                _multiplier_counts,
                &mut ore_used,
            );
            fuel += 1;
        }
        // the loop overshoots by 1
        fuel - 1
    }

    fn calc_ore_inner(
        &self,
        output_id: String,
        needed_quantity: u64,
        available: &mut HashMap<String, u64>,
        multiplier_counts: &mut HashMap<String, u64>,
        ore_used: &mut u64,
    ) {
        let output_eq = self.equations_by_output.get(&output_id);
        match output_eq {
            Some(oe) => {
                // need needed_quantity of output_id
                let available_quantity = *available.get(&output_id).unwrap_or(&0);

                if available_quantity >= needed_quantity {
                    // we have enough of the output already
                    let new_available = available_quantity - needed_quantity;
                    available.insert(output_id.clone(), new_available);
                } else {
                    // we need to produce more
                    let remaining_needed = needed_quantity - available_quantity;
                    let multiplier = oe.multiplier_to_produce(remaining_needed);

                    multiplier_counts
                        .entry(output_id.clone())
                        .and_modify(|e| *e += multiplier)
                        .or_insert(multiplier);

                    let new_produced: u64 = oe.output.quantity * multiplier;
                    available.insert(
                        output_id.clone(),
                        new_produced + available_quantity - needed_quantity,
                    );

                    // calc recursively for each input
                    oe.inputs.iter().for_each(|input| {
                        self.calc_ore_inner(
                            input.id.clone(),
                            input.quantity * multiplier,
                            available,
                            multiplier_counts,
                            ore_used,
                        )
                    });
                }
            }
            None => {
                // no equation --> ore
                *ore_used += needed_quantity;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Equation {
    multiplier: u64,
    inputs: Vec<EqComponent>,
    output: EqComponent,
}

impl Equation {
    fn multiplier_to_produce(&self, desired_quantity: u64) -> u64 {
        let mult = desired_quantity / self.output.quantity;
        if desired_quantity % self.output.quantity != 0 {
            mult + 1
        } else {
            mult
        }
    }
}

#[derive(Debug, Clone)]
struct EqComponent {
    quantity: u64,
    id: String,
}

fn parse_input(input: &str) -> SystemOfEquations {
    let regex = Regex::new(r"(\d+) ([A-Z]+)").unwrap();

    // read equations in, (key = output, value = input)
    let mut system = SystemOfEquations {
        equations_by_output: HashMap::new(),
    };

    input.lines().for_each(|line| {
        let mut eq_components: Vec<EqComponent> = Vec::new();
        regex.captures_iter(line).for_each(|component| {
            let quantity = &component[1].parse::<u64>().unwrap();
            let id = &component[2];
            eq_components.push(EqComponent {
                id: id.to_string().clone(),
                quantity: *quantity,
            });
        });

        let output = eq_components.pop().unwrap();
        let equation = Equation {
            multiplier: 1,
            inputs: eq_components,
            output,
        };
        system.add_eq(equation);
    });
    system
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, INPUT};

    const INPUT_EX3: &str = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_one() {
        let input = "9 ORE => 2 A
                            8 ORE => 3 B
                            7 ORE => 5 C
                            3 A, 4 B => 1 AB
                            5 B, 7 C => 1 BC
                            4 C, 1 A => 1 CA
                            2 AB => 1 FUEL";
        assert_eq!(part1(input), 51);
    }

    #[test]
    fn test_two() {
        let input = "9 ORE => 2 A
                            8 ORE => 3 B
                            7 ORE => 5 C
                            3 A, 4 B => 1 AB
                            5 B, 7 C => 1 BC
                            4 C, 1 A => 1 CA
                            2 AB, 3 A => 1 FUEL";
        assert_eq!(part1(input), 51 + 18);
    }

    #[test]
    fn test_three() {
        let input = "9 ORE => 2 A
                            8 ORE => 3 B
                            7 ORE => 5 C
                            3 A, 4 B => 1 AB
                            5 B, 7 C => 1 BC
                            4 C, 1 A => 1 CA
                            2 AB, 3 BC => 1 FUEL";
        // 2 AB n 6A, 8B
        // 6A n 27 ORE
        // 8B n 24 ORE

        // 3BC n 21C 15B
        // 21C n 35 ORE
        // 15B n 40 ORE
        assert_eq!(part1(input), 51 + 75);
    }

    #[test]
    fn test4() {
        let input = "157 ORE => 5 NZVS
                            165 ORE => 6 DCFZ
                            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                            179 ORE => 7 PSHF
                            177 ORE => 5 HKGWZ
                            7 DCFZ, 7 PSHF => 2 XJWVT
                            165 ORE => 2 GPVTF
                            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(part1(input), 13312);
    }

    #[test]
    fn test5() {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                            17 NVRVD, 3 JNWZP => 8 VPVL
                            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                            22 VJHF, 37 MNCFX => 5 FWMGM
                            139 ORE => 4 NVRVD
                            144 ORE => 7 JNWZP
                            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                            145 ORE => 6 MNCFX
                            1 NVRVD => 8 CXFTF
                            1 VJHF, 6 MNCFX => 4 RFSQX
                            176 ORE => 6 VJHF";
        assert_eq!(part1(input), 180697);
    }

    #[test]
    fn test6() {
        assert_eq!(part1(INPUT_EX3), 2210736);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 143173);
    }

    #[test]
    fn test_part2_ex3() {
        assert_eq!(part2(INPUT_EX3), 460664);
    }

    #[test]
    #[ignore] // too slow...
    fn test_part2() {
        assert_eq!(part2(INPUT), 8845261);
    }
}
