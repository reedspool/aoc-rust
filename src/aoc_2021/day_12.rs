use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Cave {
    big: bool,
    start: bool,
    end: bool,
}

#[allow(dead_code)]
pub fn part_1(input: String) -> i64 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut caves: HashMap<&str, Cave> = HashMap::new();
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let edges = line.trim().split("-").collect::<Vec<&str>>();

        let first = edges[0];
        caves.insert(
            first,
            Cave {
                big: first == first.to_ascii_uppercase(),
                start: first == "start",
                end: first == "end",
            },
        );

        let second = edges[1];
        caves.insert(
            second,
            Cave {
                big: second == second.to_ascii_uppercase(),
                start: second == "start",
                end: second == "end",
            },
        );

        match paths.get_mut(first) {
            Some(terminals) => terminals.push(second),
            None => {
                let mut terminals = Vec::new();
                terminals.push(second);
                paths.insert(first, terminals);
            }
        }

        match paths.get_mut(second) {
            Some(terminals) => terminals.push(first),
            None => {
                let mut terminals = Vec::new();
                terminals.push(first);
                paths.insert(second, terminals);
            }
        }
    }

    //println!("Caves {:?}\n\nPaths {:?}", caves, paths);

    let mut stack = Vec::new();
    let mut visited: Vec<&str> = Vec::new();
    visited.push("start");

    for terminal in paths.get("start").unwrap() {
        stack.push((terminal, visited.clone()));
    }

    let mut total_paths = 0;
    let mut infinite_loop_trap = 1000000;
    while !stack.is_empty() {
        if (infinite_loop_trap == 0) {
            // panic!();
        }
        infinite_loop_trap -= 1;
        let (current, mut visited) = stack.pop().unwrap();
        //println!("Popping {}", current);
        //println!("Stack is now {:?}", stack);
        if current == &"end" {
            total_paths += 1;
            //println!("Found a path! Total: {}", total_paths);
            continue;
        }

        if visited.contains(current) && !caves.get(current).unwrap().big {
            //println!("Pushed a visited node {}", current);
            continue;
        }

        visited.push(current);
        //println!("New visited is {:?}", visited);

        for terminal in paths.get(current).unwrap() {
            if visited.contains(terminal) && !caves.get(terminal).unwrap().big {
                //println!("Not visiting {} because I already did", terminal);
                continue;
            }

            stack.push((terminal, visited.clone()));
        }
    }

    total_paths
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i64 {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut caves: HashMap<&str, Cave> = HashMap::new();
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let edges = line.trim().split("-").collect::<Vec<&str>>();

        let first = edges[0];
        caves.insert(
            first,
            Cave {
                big: first == first.to_ascii_uppercase(),
                start: first == "start",
                end: first == "end",
            },
        );

        let second = edges[1];
        caves.insert(
            second,
            Cave {
                big: second == second.to_ascii_uppercase(),
                start: second == "start",
                end: second == "end",
            },
        );

        match paths.get_mut(second) {
            Some(terminals) => terminals.push(first),
            None => {
                let mut terminals = Vec::new();
                terminals.push(first);
                paths.insert(second, terminals);
            }
        }

        match paths.get_mut(first) {
            Some(terminals) => terminals.push(second),
            None => {
                let mut terminals = Vec::new();
                terminals.push(second);
                paths.insert(first, terminals);
            }
        }

    }

    //println!("Caves {:?}\n\nPaths {:?}", caves, paths);

    let mut stack = Vec::new();
    let mut visited: Vec<&str> = Vec::new();
    visited.push("start");

    for terminal in paths.get("start").unwrap() {
        stack.push((terminal, visited.clone(), false));
    }

    let mut total_paths = 0;
    let mut infinite_loop_trap = 10000;
    while !stack.is_empty() {
        if infinite_loop_trap == 0 {
            panic!("Total so far: {}", total_paths);
        }
        infinite_loop_trap -= 1;
        let (current, mut visited, already_visited_small_twice) = stack.pop().unwrap();
        // println!("???? {} {}", already_visited_small_twice, stack.len());
        if already_visited_small_twice {
            // println!("A second visit thread");
        }
        if visited.join(",") == "start,b,A,c,A" {
            // println!("LOOK HERE!");
        }

        //println!("Popping {}", current);
        if current == &"start" {
            continue;
        }
        if current == &"end" {
            visited.push("end");
            println!("End visited is {:?}", visited.join(","));

            total_paths += 1;
            //println!("Found a path! Total: {}", total_paths);
            continue;
        }

        // if visited.contains(current) && !caves.get(current).unwrap().big {
        //     //println!("Pushed a visited node {}", current);
        //     continue;
        // }

        visited.push(current);
        // println!("New visited is {:?}", visited);
        // println!("Stack is now {:?}", stack);

        for terminal in paths.get(current).unwrap() {
            if terminal == &"start" {
                continue;
            }
            if visited.contains(terminal) && !caves.get(terminal).unwrap().big {
                if already_visited_small_twice {
                    // println!(
                    //     "Not going to {} because it would be a second duplicate",
                    //     terminal
                    // );
                    continue;
                } else {
                    // println!("We did get here");
                    stack.push((terminal, visited.clone(), true));
                }

                //println!("Not visiting {} because I already did", terminal);
            } else {
                stack.push((terminal, visited.clone(), already_visited_small_twice));
            }
        }
    }

    total_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_12_1");

        let result = part_1(input.clone());

        //println!("My answer is {}", result);
        assert_eq!(result, 5076);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = read_input("aoc_2021_12_1");

    //     let result = part_2(input.clone());

    //     //println!("My answer is {}", result);
    //     assert_eq!(result, 0);
    // }

    #[test]
    fn test_part_1_example_1() {
        let input = String::from(
            "
start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
        );

        let result = part_1(input.clone());

        //println!("My answer is {}", result);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part_2_example_1() {
        let input = String::from(
            "
start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
        );

        let result = part_2(input.clone());

        //println!("My answer is {}", result);
        assert_eq!(result, 103);
    }
}
