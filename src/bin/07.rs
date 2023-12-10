use crate::Card::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};
use crate::Hand::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair};
use itertools::{Itertools};
use std::cmp::Ordering;
use std::iter::zip;
advent_of_code::solution!(7);

#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard,     //5
    OnePair,      //4
    TwoPair,      //3
    ThreeOfAKind, //3
    FullHouse,    //2
    FourOfKind,   //2
    FiveOfKind,   //1
}

#[derive(Debug)]
struct HandStruct {
    cards: [Card; 5],
    best: Hand,
    bid: u32,
    with_joker: bool,
}

impl HandStruct {
    fn new(input: &str, with_joker: bool) -> Self {
        let (hand_str, bid) = input.split_once(' ').expect("Meep input bad");
        let mut iter = hand_str.chars();
        let cards: [Card; 5] = std::array::from_fn(|_| {
            let card_c = iter.next().expect("too short");
            match card_c {
                '2' => Two,
                '3' => Three,
                '4' => Four,
                '5' => Five,
                '6' => Six,
                '7' => Seven,
                '8' => Eight,
                '9' => Nine,
                'T' => Ten,
                'J' => Jack,
                'Q' => Queen,
                'K' => King,
                'A' => Ace,
                _ => {
                    println!("Hey, you, your input is bad: {card_c}");
                    Two
                }
            }
        });
        HandStruct {
            cards,
            bid: bid.parse().expect("Bid Bad"),
            best: HandStruct::get_best(&cards, with_joker),
            with_joker,
        }
    }

    fn get_best(cards: &[Card; 5], with_joker: bool) -> Hand {
        let joker_count = cards.iter().filter(|x| **x == Jack).count();
        let cards = if !with_joker || joker_count == 5 {
            cards.to_vec()
        } else {
            cards
                .iter()
                .filter_map(|x| {
                    if *x == Jack {
                        return None;
                    }
                    Some(*x)
                })
                .collect()
        };
        let sorted_cards = cards.iter().sorted();
        let dedup_with_count: Vec<_> = sorted_cards.dedup_with_count().collect();
        let size_largest_group = dedup_with_count
            .iter()
            .max_by(|x, y| x.0.cmp(&y.0))
            .expect("Count failed")
            .0;
        let size_largest_group = if with_joker {
            size_largest_group + joker_count
        } else {
            size_largest_group
        };
        let unique_count = dedup_with_count.len();
        if size_largest_group >= 5 {
            return FiveOfKind;
        }
        if size_largest_group == 4 && unique_count == 2 {
            return FourOfKind;
        }
        if size_largest_group == 3 && unique_count == 2 {
            return FullHouse;
        }
        if size_largest_group == 3 && unique_count == 3 {
            return ThreeOfAKind;
        }
        if unique_count == 3 {
            return TwoPair;
        }
        if unique_count == 4 {
            return OnePair;
        }
       HighCard
    }
}

impl Ord for HandStruct {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.best != other.best {
            return self.best.cmp(&other.best);
        }
        for (my, other) in zip(self.cards, other.cards) {
            if self.with_joker {
                if my == Jack && other == Jack {
                    continue;
                }
                if my == Jack {
                    return Ordering::Less;
                }
                if other == Jack {
                    return Ordering::Greater;
                }
            }
            if my > other {
                return Ordering::Greater;
            }
            if my < other {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for HandStruct {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandStruct {
    fn eq(&self, other: &Self) -> bool {
        (self.cards) == (other.cards)
    }
}

impl Eq for HandStruct {}

pub fn part_one(input: &str) -> Option<u32> {
    let hands: Vec<_> = input.lines().map(|x| HandStruct::new(x, false)).collect();
    let sorted_hands: Vec<_> = hands.iter().sorted().collect();
    let result: u32 = sorted_hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands: Vec<_> = input.lines().map(|x| HandStruct::new(x, true)).collect();
    let sorted_hands: Vec<_> = hands.iter().sorted().collect();
    let result: u32 = sorted_hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u32 + 1) * hand.bid)
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_ordering() {
        assert_eq!(OnePair.cmp(&TwoPair), Ordering::Less);
        assert_eq!(Hand::OnePair == Hand::OnePair, true);
    }

    #[test]
    fn test_handstruct() {
        let hand = HandStruct::new("JJJJJ 100", true);
        assert_eq!(hand.best, FiveOfKind);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
