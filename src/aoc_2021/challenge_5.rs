use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Hash)]
struct Line {
    beg: Point,
    end: Point,
}

#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let input_lines = input.trim().split("\n");

    let lines = input_lines
        .map(|line| {
            let line = line
                .trim()
                .split(" -> ")
                .map(|point| {
                    // println!("Parsing Point: {} ", point);
                    let xys = point
                        .split(",")
                        .map(|value| value.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    Point {
                        x: xys[0],
                        y: xys[1],
                    }
                })
                .collect::<Vec<Point>>();

            Line {
                beg: line[0],
                end: line[1],
            }
        })
        .collect::<Vec<Line>>();

    let mut grid = HashMap::<Point, i32>::new();

    let mut results = HashSet::<Point>::new();

    lines.iter().for_each(|line| {
        if !(line.beg.x == line.end.x || line.beg.y == line.end.y) {
            return;
        }

        // Vertical
        if line.beg.x == line.end.x {
            let start = if line.beg.y < line.end.y {
                line.beg
            } else {
                line.end
            };
            let end = if line.beg.y > line.end.y {
                line.beg
            } else {
                line.end
            };
            for y in start.y..=end.y {
                let point = Point {
                    y: y,
                    x: line.beg.x,
                };

                match grid.get(&point) {
                    Some(value) => {
                        grid.insert(point, value + 1);
                        results.insert(point);
                    }
                    None => {
                        grid.insert(point, 1);
                    }
                }
            }
        }

        // Horizontal
        if line.beg.y == line.end.y {
            let start = if line.beg.x < line.end.x {
                line.beg
            } else {
                line.end
            };
            let end = if line.beg.x > line.end.x {
                line.beg
            } else {
                line.end
            };

            for x in start.x..=end.x {
                let point = Point {
                    x: x,
                    y: line.beg.y,
                };

                match grid.get(&point) {
                    Some(value) => {
                        grid.insert(point, value + 1);
                        results.insert(point);
                    }
                    None => {
                        grid.insert(point, 1);
                    }
                }
            }
        }
    });

    // println!("Grid {:?}", grid);
    // println!("Results {:?}", results);

    results.len() as i32
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let input_lines = input.trim().split("\n");

    let lines = input_lines
        .map(|line| {
            let line = line
                .trim()
                .split(" -> ")
                .map(|point| {
                    // println!("Parsing Point: {} ", point);
                    let xys = point
                        .split(",")
                        .map(|value| value.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

                    Point {
                        x: xys[0],
                        y: xys[1],
                    }
                })
                .collect::<Vec<Point>>();

            Line {
                beg: line[0],
                end: line[1],
            }
        })
        .collect::<Vec<Line>>();

    let mut grid = HashMap::<Point, i32>::new();

    let mut results = HashSet::<Point>::new();

    lines.iter().for_each(|line| {
        // Vertical
        if line.beg.x == line.end.x && line.beg.y != line.end.y {
            let start = if line.beg.y < line.end.y {
                line.beg
            } else {
                line.end
            };
            let end = if line.beg.y > line.end.y {
                line.beg
            } else {
                line.end
            };
            for y in start.y..=end.y {
                let point = Point {
                    y: y,
                    x: line.beg.x,
                };

                match grid.get(&point) {
                    Some(value) => {
                        grid.insert(point, value + 1);
                        results.insert(point);
                    }
                    None => {
                        grid.insert(point, 1);
                    }
                }
            }
        }

        // Horizontal
        if line.beg.y == line.end.y {
            let start = if line.beg.x < line.end.x {
                line.beg
            } else {
                line.end
            };
            let end = if line.beg.x > line.end.x {
                line.beg
            } else {
                line.end
            };

            for x in start.x..=end.x {
                let point = Point {
                    x: x,
                    y: line.beg.y,
                };

                match grid.get(&point) {
                    Some(value) => {
                        grid.insert(point, value + 1);
                        results.insert(point);
                    }
                    None => {
                        grid.insert(point, 1);
                    }
                }
            }
        }

        if line.beg.x == line.end.x || line.beg.y == line.end.y {
            return;
        }

        // Diagonal
        let stepX: i32 = match line.beg.x < line.end.x {
            true => 1,
            false => -1,
        };

        let stepY: i32 = match line.beg.y < line.end.y {
            true => 1,
            false => -1,
        };

        let mut moving_point = line.beg.clone();

        loop {
            match grid.get(&moving_point) {
                Some(value) => {
                    grid.insert(moving_point.clone(), value + 1);
                    results.insert(moving_point.clone());
                }
                None => {
                    grid.insert(moving_point.clone(), 1);
                }
            }

            // println!("Moving point: {:?}", moving_point);
            moving_point.x += stepX;
            moving_point.y += stepY;

            if moving_point == line.end {
                match grid.get(&moving_point) {
                    Some(value) => {
                        grid.insert(moving_point.clone(), value + 1);
                        results.insert(moving_point.clone());
                    }
                    None => {
                        grid.insert(moving_point.clone(), 1);
                    }
                }
                break;
            }
        }
    });

    // println!("Grid {:?}", grid);
    // println!("Results {:?}", results);

    results.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_5_1_input");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 6005);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_5_1_input");

        let result = part_2(input.clone());

        assert!(result != 23837);
        println!("My answer is {}", result);
        assert_eq!(result, 23864);
    }

    #[test]
    fn test_part_1_example() {
        let input = String::from(
            "
               0,9 -> 5,9
               8,0 -> 0,8
               9,4 -> 3,4
               2,2 -> 2,1
               7,0 -> 7,4
               6,4 -> 2,0
               0,9 -> 2,9
               3,4 -> 1,4
               0,0 -> 8,8
               5,5 -> 8,2
            ",
        );

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_2_example() {
        let input = String::from(
            "
               0,9 -> 5,9
               8,0 -> 0,8
               9,4 -> 3,4
               2,2 -> 2,1
               7,0 -> 7,4
               6,4 -> 2,0
               0,9 -> 2,9
               3,4 -> 1,4
               0,0 -> 8,8
               5,5 -> 8,2
            ",
        );

        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_range_iterator_with_1_value() {
        let mut result = 0;
        for i in 1..=1 {
            result += 1;
        }

        assert_eq!(result, 1);
    }
}
