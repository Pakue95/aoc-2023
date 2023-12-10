use itertools::Itertools;
advent_of_code::solution!(6);

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let mut tup_it = input.lines().tuples::<(_,_)>();
    let (time_str, dist_str) = tup_it.next().unwrap();

    let mut time_str_it = time_str.split_whitespace();
    let mut dist_str_it = dist_str.split_whitespace();

    time_str_it.next();
    dist_str_it.next();

    time_str_it.zip(dist_str_it).map(|(time, dist)|{
        let out: (u32, u32) = (time.parse().unwrap(), dist.parse().unwrap());
        out
    }).collect()
}

fn find_max_options(time: &u32, distance: &u32) -> u32 {
    let options : u32 = (0..time+1).map(|x| {
        (time-x)*x
    })
        .filter(|x| *x > *distance)
        .count() as u32;
    options
}

fn find_max_options_bin(time: &u64, distance: &u64) -> u64 {
    let mut lower_bound = *time /2;
    let mut left = 0_u64;
    let mut right = *time;

    loop {
        let is_in_winning = (time-lower_bound) * lower_bound > *distance;
        let is_lower_border= (time-(lower_bound-1)) * (lower_bound-1) <= *distance;

        if is_in_winning && is_lower_border {
            break
        }
        // println!("{lower_bound}");

        if (time-lower_bound) * lower_bound > *distance {
            right = lower_bound;
            lower_bound = (lower_bound+left) / 2;
            // lower_bound = left + (lower_bound - left) / 2;
            continue
        }
        else {
            left = lower_bound;
            lower_bound = (lower_bound+right) / 2;
            // lower_bound = lower_bound + (right - lower_bound) / 2;
        }
    }
    // println!("low: {lower_bound}");

    let mut upper_bound= *time/2;
    let mut left = 0;
    let mut right = *time;

    loop {
        let is_in_winning = (time-upper_bound) * upper_bound > *distance;
        let is_upper_border= (time-(upper_bound+1)) * (upper_bound+1) <= *distance;

        if is_in_winning && is_upper_border {
            break
        }
        // println!("{upper_bound}");

        if (time-upper_bound) * upper_bound > *distance {
            left = upper_bound;
            upper_bound = (upper_bound+right) / 2;
            // upper_bound = left + (upper_bound - left) / 2;
            continue
        }
        else {
            right = upper_bound;
            upper_bound = (upper_bound+left) / 2;
            // upper_bound = upper_bound + (right - upper_bound) / 2;
        }
    }
    // println!("high: {upper_bound}");
    upper_bound-lower_bound+1
}

pub fn part_one(input: &str) -> Option<u32> {

    let times = parse_input(input);
    // println!("{times:?}");
    let maxis = times.iter().map(|(time, distance)| {
        find_max_options(time, distance)
    }).product::<u32>();
    Some(maxis)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut tup_it = input.lines().tuples::<(_,_)>();
    let (time_str, dist_str) = tup_it.next().unwrap();

    let (_, time_str) = time_str.split_once(':').unwrap();
    let (_, dist_str) = dist_str.split_once(':').unwrap();

    let dist_str: String = dist_str.chars().filter(|x| !x.is_whitespace()).collect();
    let time_str: String = time_str.chars().filter(|x| !x.is_whitespace()).collect();

    // println!("dist {dist_str}, time {time_str}");

    let time: u64 = time_str.parse().unwrap();
    let dist: u64 = dist_str.parse().unwrap();

    // println!("dist {dist}, time {time}");
    // println!("test {}", find_max_options_bin(&15, &40));

    let result = find_max_options_bin(&time, &dist);
    // println!("{result}");

    Some(result)
    // find_max_options_bin(&71530, &940200);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
