use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use itertools::Itertools;
use prime_factorization::Factorization;

advent_of_code::solution!(8);

#[derive(Debug)]
struct GhostPath {
    loop_size: usize,
}
impl GhostPath {
    fn new(start: &str, path_str: & str, camel_map: & HashMap<&str, (&str, &str)>) -> Option<Self> {
        let mut pos = start;
        let mut visited: Vec<&str> = Vec::with_capacity(20000);
        let mut visited_hash: HashSet<(&str, usize)> = HashSet::new();
        let path_str_len = path_str.len();

        for (idx, dir) in path_str.chars().cycle().enumerate() {
            let path_str_idx = idx % path_str_len;

            if visited_hash.contains(&(pos, path_str_idx)) {
                let path = GhostPath {
                    loop_size: visited
                        .iter()
                        .skip_while(|&&visit| visit != pos)
                        .count(),
                };
                return Some(path)
            }
            visited.push(pos);
            visited_hash.insert((pos, path_str_idx));

            let (left, right) = camel_map.get(pos).expect("Ey, no map!");

            if dir == 'R' {
                pos = *right
            }
            else if dir == 'L' {
                pos = *left
            }
            else {
                println!("Ops");
            }
        }
        None
    }
}
// https://en.wikipedia.org/wiki/Least_common_multiple
fn least_common_multiple(numbers: Vec<u64>) -> u64{
    if numbers.len() == 1 {
        return numbers[0];
    }
    let max_prime_pows: Vec<(usize, u64)> = highest_pow_of_prim_factors(numbers);

    let result = max_prime_pows.iter()
        .fold(1,|accu, (expo, base)| {
            accu * base.pow(*expo as u32)
        });
   result
}

fn highest_pow_of_prim_factors(numbers: Vec<u64>) -> Vec<(usize, u64)>{
    numbers.iter().flat_map(|num|{
        if *num == 1 {return vec!((1_usize,1_u64))}

        let factors = Factorization::<u64>::run(*num).factors;

        factors.into_iter()
            .sorted()
            .dedup_with_count() // (exponent, base)
            .collect_vec()
    })
        .sorted_by(|(a_expo, a_base), (b_expo, b_base)| {
            a_base.cmp(b_base)
                .then(a_expo.cmp(b_expo))
        })
        .rev()
        .dedup_by(|x, y| x.1 == y.1)
        .collect_vec()
}


pub fn part_one(input: &str) -> Option<u32> {
    let (path_str, map_str) = input.split_once("\n\n").expect("Yo, Input");

    let mut camel_map: HashMap<&str, (&str, &str)> = HashMap::new();

    for i in map_str.lines() {
        let index = &i[..3];
        let left = &i[7..10];
        let right = &i[12..15];

        camel_map.insert(index, (left, right));

    }
    let mut pos = "AAA";

    for (idx, dir) in path_str.chars().cycle().enumerate(){
        let (left, right) = camel_map.get(pos).expect("Ey, no map!");

        if pos == "ZZZ" && idx > 0 {
            // println!("Back at start after: {idx}");
            return Some(idx as u32)
        }
        if dir == 'R' {
            pos = *right
        }
        else if dir == 'L' {
            pos = *left
        }
        else {
            println!("Ops");
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {

    let (path_str, map_str) = input.split_once("\n\n").expect("Yo, Input");
    let mut camel_map: HashMap<&str, (&str, &str)> = HashMap::new();

    for i in map_str.lines() {
        let index = &i[..3];
        let left = &i[7..10];
        let right = &i[12..15];

        camel_map.insert(index, (left, right));

    }
    let pos: Vec<_> = camel_map.clone()
        .into_keys()
        .filter(|x| x.chars().last().expect("AAAA") == 'A')
        .collect();

    let ghost_paths: Vec<_> = pos.par_iter().map(|x|{
        GhostPath::new(x, path_str, &camel_map).unwrap()
    }).collect();

    let ghost_loops: Vec<u64> = ghost_paths
        .iter()
        .map(|x| x.loop_size as u64)
        .collect();


    Some(least_common_multiple(ghost_loops) )
}

#[cfg(test)]
mod tests {
    use timeit::timeit;
    use timeit::timeit_loops;

    use super::*;

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(12083));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(12083));
    }

    #[test]
    fn test_lcm() {
        assert_eq!(least_common_multiple(vec![1,2,3,4,5]), 60);
        assert_eq!(least_common_multiple(vec![100, 40, 21441]), 4288200);
        timeit!({
            assert_eq!(least_common_multiple(vec![22199, 17141, 13207, 18827, 20513, 12083]), 13385272668829);
        });

    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }
}
