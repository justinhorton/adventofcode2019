const INPUT: &str = include_str!("../day1.txt");

fn main() {
    println!("Day 1-1: {}", get_total_fuel(calc_fuel_mass_only));
    println!("Day 1-2: {}", get_total_fuel(calc_fuel_package_and_fuel));
}

fn get_total_fuel(fuel_fn: fn(i32) -> i32) -> i32 {
    INPUT
        .lines()
        .map(|line| line.parse::<i32>().expect(&format!("Can't parse int")))
        .map(fuel_fn)
        .sum()
}

// fuel calculation
fn calc_fuel_mass_only(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calc_fuel_package_and_fuel(mass: i32) -> i32 {
    let mut result = 0;
    let mut cur_fuel = calc_fuel_mass_only(mass);

    while cur_fuel > 0 {
        result += cur_fuel;
        cur_fuel = calc_fuel_mass_only(cur_fuel);
    }
    result
}
