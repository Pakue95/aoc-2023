use itertools::Itertools;
use rayon::prelude::*;
advent_of_code::solution!(9);

fn get_differences(nums: &Vec<i64>) -> Vec<i64> {
    nums.windows(2).map(|x| x[1] - x[0]).collect()
}

fn as_numbers(input: &str) -> Vec<Vec<i64>> {
    let mut result = Vec::with_capacity(200);
    for line in input.lines() {
        let mut row = Vec::with_capacity(21);
        for num in line.split_whitespace(){
            row.push(num.parse::<i64>().expect("INPUT!"));
        }
        result.push(row);
    }
    result
}

fn make_prediction_back(input: &Vec<i64>) -> i64 {
    let mut last_digits: Vec<i64> = Vec::with_capacity(input.len());
    let mut cur_row = input.clone();
    loop {
        cur_row = get_differences(&cur_row);
        last_digits.push(*cur_row.last().expect("No last digit!"));
        if cur_row.iter().all_equal() {
            break
        }
    }
    input.last().expect("No Back!")
        + last_digits
            .into_iter()
            .fold(0, |acc, e| acc + e)
}

fn make_prediction_front(input: &Vec<i64>) -> i64 {
    let mut first_digits: Vec<i64> = Vec::with_capacity(input.len());
    let mut cur_row = input.clone();
    loop {
        cur_row = get_differences(&cur_row);
        first_digits.push(*cur_row.first().expect("No last digit!"));
        if cur_row.iter().all(|x| *x == 0) {
            break;
        }
    }
    input[0]
        - first_digits
            .into_iter()
            .rfold(0, |acc, e| e - acc)
}

pub fn part_one(input: &str) -> Option<i64> {

        let as_numbers: Vec<Vec<i64>> = as_numbers(input);

        Some(as_numbers
            .into_par_iter()
            .map(|x|make_prediction_back(&x))
            .sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let as_numbers: Vec<Vec<i64>> = as_numbers(input);

    Some(as_numbers
        .into_par_iter()
        .map(|x|make_prediction_front(&x))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
