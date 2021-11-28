const START: i32 = 271973;
const END: i32 = 785961;

fn part_1() -> i32 {
    let mut current = START;
    let mut count = 0;
    let mut current_string: &str;

    while current <= END {
        let mut current_string = &current.to_string();
        if contains_pairs(current_string) {
            let test = next_decrease(current_string);

            match test {
                Some(next) => {
                    current = find_next_increase(current_string, next);
                    continue;
                }
                None => {
                    count += 1;
                }
            }
        }

        current += 1;
    }

    count
}

fn part_2() -> i32 {
    let mut current = START;
    let mut count = 0;
    let mut current_string: &str;

    while current <= END {
        let mut current_string = &current.to_string();
        if contains_pairs_no_triples(current_string) {
            let test = next_decrease(current_string);

            match test {
                Some(next) => {
                    current = find_next_increase(current_string, next);
                    continue;
                }
                None => {
                    count += 1;
                }
            }
        }

        current += 1;
    }

    count
}

fn contains_pairs(value: &str) -> bool {
    let mut last = 0;
    for (index, byte) in value.bytes().enumerate() {
        if index != 0 && last == byte {
            return true;
        }

        last = byte;
    }

    false
}
fn contains_pairs_no_triples(value: &str) -> bool {
    let mut last = 0;
    let mut found_pair_last = false;
    let mut found_too_many = false;

    for (index, current) in value.bytes().enumerate() {
        if found_too_many {
            if current == last {
                continue;
            } else {
                found_too_many = false;
                found_pair_last = false;
            }
        }
        if index >= 1 {
            if found_pair_last {
                if last != current {
                    return true;
                } else {
                    found_pair_last = false;
                    found_too_many = true;
                    continue;
                }
            }
            if last == current {
                found_pair_last = true;
            }
        }

        last = current;
    }

    found_pair_last
}

fn next_decrease(value: &str) -> Option<(usize, u8)> {
    let mut last = (0, 0);
    for (index, current) in value.bytes().enumerate() {
        if index != 0 && last.1 > current {
            return Some(last);
        }

        last = (index, current);
    }

    None
}

fn find_next_increase(value: &str, place: (usize, u8)) -> i32 {
    let first_index = place.0 + 1;
    let mut next = String::from(&value[0..first_index]);

    next.push(place.1 as char);

    let last_index = value.len();

    for _ in (first_index + 1)..last_index {
        next.push('0');
    }

    next.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), 925);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), 607);
    }

    #[test]
    fn test_adj_same() {
        assert_eq!(contains_pairs("111111"), true);
        assert_eq!(contains_pairs("223450"), true);
        assert_eq!(contains_pairs("123789"), false);
    }

    #[test]
    fn test_contains_no_triples() {
        assert_eq!(contains_pairs_no_triples("112233"), true);
        assert_eq!(contains_pairs_no_triples("123444"), false);
        assert_eq!(contains_pairs_no_triples("111122"), true);
    }

    #[test]
    fn test_monotonically_increasing_digits() {
        assert_eq!(next_decrease("111111"), None);
        assert_eq!(next_decrease("223450"), Some((4, b'5')));
        assert_eq!(next_decrease("203450"), Some((0, b'2')));
        assert_eq!(next_decrease("123789"), None);
    }

    #[test]
    fn test_find_next_increase() {
        assert_eq!(find_next_increase("223450", (4, b'5')), 223455);
        assert_eq!(find_next_increase("203450", (0, b'2')), 220000);
    }
}
