enum directions {
    forward,
    up,
    down
}

#[allow(dead_code)]
pub fn part_1(input: String) -> i32 {
    let instructions = input.trim()
        .split("\n")
        .map(|line| {
            let mut line = line.split(" ").into_iter();
            let direction = match line.next().unwrap() {
                "forward" => directions::forward,
                "up"=> directions::up,
                "down"=> directions::down,
                _ => panic!()
            };

            let amount : i32 = line.next().unwrap().parse().unwrap();

            (direction, amount)
        });

    let mut depth = 0;
    let mut x = 0;
    for (direction, amount) in instructions {
        match direction {
            directions::forward => { x += amount },
            directions::up => { depth -= amount },
            directions::down => { depth += amount },
        }
    }

    x * depth
}

#[allow(dead_code)]
pub fn part_2(input: String) -> i32 {
    let instructions = input.trim()
        .split("\n")
        .map(|line| {
            let mut line = line.split(" ").into_iter();
            let direction = match line.next().unwrap() {
                "forward" => directions::forward,
                "up"=> directions::up,
                "down"=> directions::down,
                _ => panic!()
            };

            let amount : i32 = line.next().unwrap().parse().unwrap();

            (direction, amount)
        });

    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;
    for (direction, amount) in instructions {
        match direction {
            directions::forward => {
                x += amount;
                depth += aim * amount;
            },
            directions::up => { aim -= amount },
            directions::down => { aim += amount },
        }
    }

    x * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;
    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_2_1");

        let result = part_1(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 2102357);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_2_1");

        let result = part_2(input.clone());

        println!("My answer is {}", result);
        assert_eq!(result, 2101031224);
    }

}
