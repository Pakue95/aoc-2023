advent_of_code::solution!(4);

// More general implementation with vectors
// fn get_number_of_wins(line: &str) -> u32 {
//     let (_, numbers_str) = line.split_once(':').expect("No stuff to parse!");
//     let (winning_str, draws_str) = numbers_str.split_once('|').expect("No stuff!");
//
//     let winning: Vec<u32> = winning_str
//         .split(' ')
//         .filter_map(|x| x.parse::<u32>().ok())
//         .collect();
//
//     let draws: Vec<u32> = draws_str
//         .split(' ')
//         .filter_map(|x| x.parse::<u32>().ok())
//         .collect();
//
//     winning
//         .iter()
//         .filter(|x| draws.contains(x))
//         .copied()
//         .count() as u32
// }

fn get_number_of_wins(line: &str) -> u32 {
    let (_, numbers_str) = line.split_once(':').expect("No stuff to parse!");
    let (winning_str, draws_str) = numbers_str.split_once('|').expect("No stuff!");

    let mut winning: [u32; 16] = [0; 16];
    let mut draws: [u32; 32] = [0; 32];

    for (idx, win) in winning_str.split_whitespace().enumerate() {
        winning[idx] = win.parse::<u32>().unwrap();
    }

    for (idx, draw) in draws_str.split_whitespace().enumerate() {
        draws[idx] = draw.parse::<u32>().unwrap();
    }

    winning
        .iter()
        .take_while(|x| **x != 0)
        .filter(|x| draws.contains(x))
        .copied()
        .count() as u32
}


pub fn part_one(input: & str) -> Option<u32> {
    let result: u32 = input.lines()
        .map(get_number_of_wins)
        .map(|length| {
            if length == 0 {0} else { 2u32.pow(length - 1) }
        })
        .sum();

    Some(result)
}

pub fn part_two(input: & str) -> Option<u32> {
    let mut cards: Vec<(u32, u32)> = input.lines()
        .map(get_number_of_wins)
        .map(|x| (1, x))
        .collect();

    for card_idx in 0..cards.len() {
        let (multiplicator, n_wins) = cards[card_idx];
        for i in 1..=n_wins as usize {
            let temp = cards.get(card_idx+i);
            if temp.is_none() { continue }
            let (temp_multi, temp_wins) = temp.unwrap();
            cards[card_idx+i] = (temp_multi+multiplicator, *temp_wins)
        }
    }

    let result = cards.iter()
        .map(|(multi, _wins)| {
            multi
        })
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
