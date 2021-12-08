#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    assert_eq!(calculate_fuel_for_mass_and_fuel(14), 2);
    assert_eq!(calculate_fuel_for_mass_and_fuel(1969), 966);
    assert_eq!(calculate_fuel_for_mass_and_fuel(100756), 50346);

    let mut fuel = 0;

    for line in input.split("\n") {
        let maybe_mass = line.parse::<i32>();

        match maybe_mass {
            Ok(mass) => fuel += calculate_fuel(mass),
            Err(_) => {}
        }
    }

    fuel
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let mut fuel = 0;

    for line in input.split("\n") {
        let maybe_mass = line.parse::<i32>();

        match maybe_mass {
            Ok(mass) => fuel += calculate_fuel_for_mass_and_fuel(mass),
            Err(_) => {}
        }
    }

    fuel
}

fn calculate_fuel_for_mass_and_fuel(mass: i32) -> i32 {
    let mut fuel = calculate_fuel(mass);
    let mut calculated_fuel = fuel;
    loop {
        calculated_fuel = calculate_fuel(calculated_fuel);
        if calculated_fuel <= 0 {
            return fuel;
        }
        fuel += calculated_fuel;
    }
}

fn calculate_fuel(mass: i32) -> i32 {
    ((mass / 3) as i32) - 2
}

#[cfg(test)]
mod tests {
    use crate::advent_utilities::read_input;
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2019_1_1");

        let result = part_1(input.clone());

        println!("The fuel just for the mass needed is {}", result);
        assert_eq!(result, 3361976);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2019_1_1");

        let result = part_2(input.clone());
        assert_eq!(result, 5040085);
        println!("The total fuel needed is {}", result);
    }
}
