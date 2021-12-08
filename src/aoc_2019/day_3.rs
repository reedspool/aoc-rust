use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
enum Direction {
    Down,
    Right,
    Left,
    Up,
}

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Location {}

impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Location {
    fn clone(&self) -> Self {
        Location {
            x: self.x,
            y: self.y,
        }
    }
}

#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let mut wires = input.trim().split("\n");

    let wire_1 = wires.next().unwrap();
    let wire_2 = wires.next().unwrap();

    let directions_1 = parse_input(wire_1.to_string());
    let directions_2 = parse_input(wire_2.to_string());

    // println!("1 is {:?}", directions_1);
    // println!("2 is {:?}", directions_2);

    let wire_1_locations = walk_and_record(directions_1);
    let wire_2_locations = walk_and_record(directions_2);

    let intersections = wire_1_locations
        .intersection(&wire_2_locations)
        .collect::<Vec<&Location>>();

    let mut min = 1000000000;

    for point in intersections {
        let distance = point.x.abs() + point.y.abs();

        if distance < min {
            min = distance
        }
    }

    min
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let mut wires = input.trim().split("\n");

    let wire_1 = wires.next().unwrap();
    let wire_2 = wires.next().unwrap();

    let directions_1 = parse_input(wire_1.to_string());
    let directions_2 = parse_input(wire_2.to_string());

    // println!("1 is {:?}", directions_1);
    // println!("2 is {:?}", directions_2);

    let wire_1_locations = walk_and_record_with_delay(directions_1);
    let wire_2_locations = walk_and_record_with_delay(directions_2);

    let mut min = 1000000000;

    for (location, wire_1_delay) in wire_1_locations {
        if wire_2_locations.contains_key(&location) {
            let wire_2_delay = wire_2_locations.get(&location).unwrap();
            let delay = wire_2_delay + wire_1_delay;

            if delay < min {
                min = delay
            }
        }
    }

    min
}

fn parse_input(input: String) -> Vec<(Direction, i32)> {
    let regex_instruction = Regex::new(r"(D|U|R|L)(\d+)").unwrap();
    let intcode = input.trim().split(",").map(|code| {
        let captures = regex_instruction.captures(code).unwrap();

        let direction_str = captures.get(1).unwrap().as_str();
        let magnitude_str = captures.get(2).unwrap().as_str();
        let magnitude = magnitude_str.parse::<i32>().unwrap();

        let direction = match direction_str {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "R" => Direction::Right,
            unknown => panic!("Unknown dir {}", unknown),
        };

        (direction, magnitude)
    });
    Vec::from_iter(intcode)
}
fn walk_and_record(instructions: Vec<(Direction, i32)>) -> HashSet<Location> {
    let mut locations = HashSet::new();

    let mut current = Location { x: 0, y: 0 };
    let mut instruction_index = 0;

    while instruction_index < instructions.len() {
        let (dir, mag) = &instructions[instruction_index];
        instruction_index += 1;

        let delta = match dir {
            Direction::Down => Location { x: 0, y: 1 },
            Direction::Right => Location { x: 1, y: 0 },
            Direction::Left => Location { x: -1, y: 0 },
            Direction::Up => Location { x: 0, y: -1 },
        };

        let mut steps = 0;
        while &steps < mag {
            current.x += delta.x;
            current.y += delta.y;
            locations.insert(current.clone());
            steps += 1;
        }
    }

    locations
}

fn walk_and_record_with_delay(instructions: Vec<(Direction, i32)>) -> HashMap<Location, i32> {
    let mut locations = HashSet::new();
    let mut locations_with_delay = HashMap::new();

    let mut current = Location { x: 0, y: 0 };
    let mut instruction_index = 0;

        let mut delay = 0;
    while instruction_index < instructions.len() {
        let (dir, mag) = &instructions[instruction_index];
        instruction_index += 1;

        let delta = match dir {
            Direction::Down => Location { x: 0, y: 1 },
            Direction::Right => Location { x: 1, y: 0 },
            Direction::Left => Location { x: -1, y: 0 },
            Direction::Up => Location { x: 0, y: -1 },
        };

        let mut steps = 0;
        while &steps < mag {
            current.x += delta.x;
            current.y += delta.y;
            delay += 1;

            if !locations.contains(&current) {
                locations.insert(current.clone());
                locations_with_delay.insert(current.clone(), delay);
            }

            steps += 1;
        }
    }

    locations_with_delay
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(String::from(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        );
        assert_eq!(
            part_1(String::from(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        );

        let input = read_input("aoc_2019_3_1");

        let result = part_1(input.clone());
        assert_eq!(result, 1195);
        println!("The distance to the closest intersection is {}", result);
    }

    #[test]
    fn test_part_2() {

        assert_eq!(
            part_2(String::from(
                "R1
R1"
            )),
            2
        );

        assert_eq!(
            part_2(String::from(
                "U1,R1
R1,U1"
            )),
            4
        );

        assert_eq!(
            part_2(String::from(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
        assert_eq!(
            part_2(String::from(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );

        let input = read_input("aoc_2019_3_1");

        let result = part_2(input.clone());
        assert_eq!(result, 91518);
        println!("The distance to the closest intersection is {}", result);
    }

    #[test]
    fn test_walk_and_record() {
        let mut input = Vec::new();
        input.push((Direction::Up, 2));
        input.push((Direction::Right, 3));

        let output = walk_and_record(input);

        let mut expected = HashSet::new();
        expected.insert(Location { x: 0, y: -1 });
        expected.insert(Location { x: 0, y: -2 });
        expected.insert(Location { x: 1, y: -2 });
        expected.insert(Location { x: 2, y: -2 });
        expected.insert(Location { x: 3, y: -2 });
        println!("expected: {:?}", expected.len());
        println!("output: {:?}", output);
        assert_eq!(expected.len(), 5);
        assert_eq!(output.len(), 5);
        assert_eq!(
            expected
                .intersection(&output)
                .collect::<Vec<&Location>>()
                .len(),
            5
        );
    }
}
