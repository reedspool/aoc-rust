#[allow(dead_code)]
fn part_1(input: String) -> i32 {
    let mut floor = 0;

    for (index, &byte) in input.into_bytes().iter().enumerate() {
        match byte {
            b'(' => floor += 1,
            b')' => floor -= 1,
            unexpected => println!("Unexpected byte '{}' at index {}", unexpected, index),
        }
    }

    floor
}

#[allow(dead_code)]
fn part_2(input: String) -> usize {
    let mut floor = 0;

    for (index, &byte) in input.into_bytes().iter().enumerate() {
        match (byte, floor) {
            (_, -1) => {
                return index;
            }
            (b'(', _) => floor += 1,
            (b')', _) => floor -= 1,
            (unexpected, _) => println!("Unexpected byte '{}' at index {}", unexpected, index),
        }
    }

    0
}

mod tests {
    use crate::advent_utilities::read_input;

    #[test]
    fn test_part_1() {
        let input = read_input("aoc_2015_1_1_input");

        let result = super::part_1(input.clone());
        println!("Santa ends up on floor #{}", result);
        assert_eq!(result, 280);
    }

    #[test]
    fn test_part_2() {
        let input = read_input("aoc_2015_1_1_input");
        let result = super::part_2(input.clone());
        println!("Santa goes into the basement on floor #{}", result);
        assert_eq!(result, 1797);
    }
}
