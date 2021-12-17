use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let map: Vec<Vec<i32>> = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut smokes: Vec<i32> = Vec::new();

            for ch in line.chars() {
                smokes.push(ch.to_string().parse().unwrap());
            }

            smokes
        })
        .collect();

    let height: usize = map.len() - 1;
    let width: usize = map[0].len() - 1;

    let mut risk_sum = 0;
    for y in 0..map.len() {
        let row = &map[y];

        for x in 0..row.len() {
            let smoke = &row[x];

            if y > 0 {
                if smoke >= &map[y - 1][x] {
                    continue;
                }
            }
            if x > 0 {
                if smoke >= &map[y][x - 1] {
                    continue;
                }
            }
            if y < height {
                if smoke >= &map[y + 1][x] {
                    continue;
                }
            }
            if x < width {
                if smoke >= &map[y][x + 1] {
                    continue;
                }
            }

            println!("Adding smoke {} for total of {}", smoke, risk_sum);
            risk_sum += smoke + 1;
        }
    }

    risk_sum
}

type Point = (usize, usize);
type Basin = HashSet<Point>;
fn get_basin_index(basins: &Vec<Basin>, point: Point) -> Option<usize> {
    for (index, basin) in basins.iter().enumerate() {
        if basin.contains(&point) {
            return Some(index);
        }
    }

    None
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let map: Vec<Vec<i32>> = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut smokes: Vec<i32> = Vec::new();

            for ch in line.chars() {
                smokes.push(ch.to_string().parse().unwrap());
            }

            smokes
        })
        .collect();

    let height: usize = map.len() - 1;
    let width: usize = map[0].len() - 1;
    let mut basins: Vec<Basin> = Vec::new();

    for y in 0..=height {
        let row = &map[y];

        for x in 0..=width {
            let smoke = &row[x];
            let top: Option<Point> = if y > 0 && map[y - 1][x] != 9 {
                Some((x, y - 1))
            } else {
                None
            };
            let left: Option<Point> = if x > 0 && map[y][x - 1] != 9 {
                Some((x - 1, y))

            } else {
                None
            };
            let current: Point = (x, y);

            if *smoke == 9 {
                continue;
            }

            // Is there a top and is it not a 9 (barrier);
            match (top, left) {
                (Some(top), Some(left)) => {
                    // Both top and left are non-barrier values, so combine their
                    // basins together and add this point.
                    let top_basin_index = get_basin_index(&basins, top).unwrap();
                    let left_basin_index = get_basin_index(&basins, left).unwrap();

                    let top_basin = basins.get_mut(top_basin_index).unwrap();
                    let top_basin = top_basin.clone();

                    let left_basin = basins.get_mut(left_basin_index).unwrap();

                    left_basin.insert(current);
                    if top_basin_index == left_basin_index {
                        // We don't want to combine and remove the same one
                        println!(
                            "Adding {} @ {:?} to existing basin {:?} (both top and left)",
                            smoke, current, left_basin_index
                        );
                        continue;
                    }

                    // Merge top into left, because I just removed it
                    for point in top_basin.iter() {
                        left_basin.insert(*point);
                    }
                    println!(
                        "Adding {} @ {:?} to existing basin {:?} and combining with basin {:?}",
                        smoke, current, left_basin_index, top_basin_index
                    );
                    basins.remove(top_basin_index);
                }
                (Some(top), None) => {
                    // Add myself to top basin
                    let top_basin_index = get_basin_index(&basins, top).unwrap();
                    let mut top_basin = basins.get_mut(top_basin_index).unwrap();
                    println!(
                        "Adding {} @ {:?} to existing basin {:?}",
                        smoke, current, top_basin_index
                    );
                    top_basin.insert(current);
                }
                (None, Some(left)) => {
                    // Add myself to left basin
                    let left_basin_index = get_basin_index(&basins, left).unwrap();
                    let mut left_basin = basins.get_mut(left_basin_index).unwrap();
                    println!(
                        "Adding {} @ {:?} to existing basin {:?}",
                        smoke, current, left_basin_index
                    );
                    left_basin.insert(current);
                }
                _ => {
                    // Create a new basin
                    let mut basin: HashSet<(usize, usize)> = HashSet::new();
                    println!(
                        "Adding {} @ {:?} to new basin {:?}",
                        smoke,
                        current,
                        basins.len()
                    );
                    basin.insert(current);
                    basins.push(basin);
                }
            }
        }
    }

    let size_sorter = |a: &HashSet<(usize, usize)>,
                       b: &HashSet<(usize, usize)>|
     -> std::cmp::Ordering { b.len().cmp(&a.len()) };

    basins.sort_by(size_sorter);

    println!(
        "basin lengths {:?}",
        basins
            .iter()
            .map(|basin| basin.len() as i32)
            .collect::<Vec<i32>>()
    );

    let sizes = basins.iter().map(|basin| basin.len() as i32);

    sizes.take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_9_1");

        let result = part_1(input.clone());

        assert_ne!(result, 561);
        assert_ne!(result, 597);
        println!("My answer is {}", result);
        assert_eq!(result, 570);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_9_1");


        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 899392);
    }

    #[test]
    fn test_part_1_example() {
        let input = "
2199943210
3987894921
9856789892
8767896789
9899965678";

        let result = part_1(input.to_string());

        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_2_example() {
        let input = "
2199943210
3987894921
9856789892
8767896789
9899965678";

        let result = part_2(input.to_string());

        assert_eq!(result, 1134);
    }
}
