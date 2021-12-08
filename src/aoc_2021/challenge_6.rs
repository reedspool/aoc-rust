#[allow(dead_code)]
pub fn part_1(input: String, days: u64) -> u64 {
    let mut fish : Vec<u64> = input.trim().split(",").map(|f| f.parse().unwrap()).collect();

    for day in 0..days {
        let reproducing = fish.iter().filter(|&&f| f == 0).count();

        fish = fish.iter().map(|&f| if f == 0 { 7 } else { f }).collect();
        fish = fish.iter().map(|f| f - 1).collect();

        for newborn in 0..reproducing {
            fish.push(8);
        }


        println!("after day {}, I have {} fishies", day, fish.len());
    }

    fish.len() as u64
}

#[allow(dead_code)]
pub fn part_2(input: String, days: u64) -> u64 {
    let mut fish : Vec<usize> = input.trim().split(",").map(|f| f.parse().unwrap()).collect();

    // Histogram of fish
    let mut fishtogram = [0u64; 9];

    for f in fish {
        fishtogram[f] += 1;
    }

    for day in 0..days {
        let reproducing = fishtogram[0];

        // Set every fish at 0 to 7 (because it's about to be decremented)
        fishtogram[7] += fishtogram[0];

        // Shift down (decrementing all)
        for i in 0..8 {
            fishtogram[i] = fishtogram[i + 1];
        }

        // Add newborns as 8s
        fishtogram[8] = reproducing;
    }

    let mut result : u64 = 0;

    for count in fishtogram {
        result += count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2021_6_1_input");

        let result = part_1(input.clone(), 80);

        println!("My answer is {}", result);
        assert_eq!(result, 352872);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2021_6_1_input");

        let result = part_2(input.clone(), 256);

        println!("My answer is {}", result);
        assert_eq!(result, 1604361182149);
    }


    #[test]
    fn test_part_1_example() {
        let input = String::from("3,4,3,1,2");

println!("Day 1:");
        let result = part_1(input.clone(), 1);
        assert_eq!(result, 5);

println!("Day 18:");
        let result = part_1(input.clone(), 18);
        assert_eq!(result, 26);

println!("Day 80:");
        let result = part_1(input.clone(), 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_part_2_example() {
        let input = String::from("3,4,3,1,2");

        println!("Day 256:");

        let result = part_2(input.clone(), 256);
        assert_eq!(result, 26984457539);

    }
}
