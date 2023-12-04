advent_of_code::solution!(1);

fn add_first_last_digit(line: &str) -> u32 {
    let first = line
        .chars()
        .find(|x| x.is_ascii_digit())
        .unwrap();

    let last = line
        .chars()
        .rev()
        .find(|x| x.is_ascii_digit())
        .unwrap();

    let combine: u32 = format!("{}{}", first, last).parse::<u32>().unwrap();

    combine
}

const NUMBER_DIGIT_LUT: [(&str, &str); 9] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn replace_number_with_digit(line: &str) -> String {
    let mut result = String::with_capacity(line.len());
    for c in line.chars() {
        result.push(if c.is_ascii_digit() { c } else { ' ' })
    }
    for (entry, digit) in NUMBER_DIGIT_LUT {
        for (idx, _) in line.match_indices(entry) {
            result.replace_range(idx..idx + 1, digit);
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines()
        .map(add_first_last_digit)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(replace_number_with_digit)
        .map(|x| add_first_last_digit(&x))
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
