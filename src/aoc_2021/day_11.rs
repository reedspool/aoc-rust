use std::collections::HashMap;

type Point = (i8, i8);
type Direction = (i8, i8);
type Map = HashMap<Point, u8>;

fn build_map(input: String) -> Map {
    let mut map: Map = HashMap::new();

    let mut y = 0;
    let mut x = 0;

    input.trim().split("\n").map(|line| {
        line.trim().chars().map(|energy| {

            let energy = energy.to_string().parse::<u8>().unwrap();

            map.insert((x, y), energy);

            x += 1;
        }).collect::<()>();

        x = 0;
        y += 1;
    }).collect::<()>();

    map
}

fn step(map: &mut Map) -> u64 {
    let mut all_offsets: Vec<Direction> = Vec::new();
    all_offsets.push((-1, -1));
    all_offsets.push((-1, 0));
    all_offsets.push((-1, 1));
    all_offsets.push((0, -1));
    all_offsets.push((0, 1));
    all_offsets.push((1, -1));
    all_offsets.push((1, 0));
    all_offsets.push((1, 1));
    let all_offsets = all_offsets;
    let mut flashing: Vec<Point> = Vec::new();
    let mut flashes = 0;

    for (point, value) in map.iter_mut() {
        if *value == 9 {
            flashing.push(*point);
        }
        *value += 1;
    }
    while flashing.len() > 0 {
        let next = flashing.pop().unwrap();
        flashes += 1;

        for offset in &all_offsets {
            let p = (next.0 + offset.0, next.1 + offset.1);

            if p.0 < 0 || p.1 < 0 || p.0 > 9 || p.1 > 9 {
                continue;
            }

            let energy = map.get(&p).unwrap();

            if *energy == 9 {
                flashing.push(p);
            }

            map.insert(p, energy + 1);
        }
    }

    for (point, value) in map.iter_mut() {
        if *value > 9 {
            *value = 0
        };
    }

    flashes
}

#[allow(dead_code)]
pub fn part_1(input: String) -> u64 {
    let mut map = build_map(input);

    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut map);
    }

    flashes
}

#[allow(dead_code)]
pub fn part_2(input: String) -> usize {
    let mut map = build_map(input);

    let mut steps = 1;

    loop {
        if step(&mut map) == 100 { break; }
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_11_1");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 1669);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_11_1");

        let result = part_2(input.clone());


        assert!(result > 350);
        println!("My answer is {}", result);
        assert_eq!(result, 351);
    }
}
