advent_of_code::solution!(7);
use std::cmp::Ordering;
use itertools::Itertools;

// 2, 3, 4 ... T J Q K A
// is five of a kind
// four of a kind
// full house, 3 + 2
// three of a kind, 3, 1, 1
// two pair, aabbc
// one pair, aabcd
// high card, ...

// ChatGPT
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfKind = 4,
    FullHouse = 5,
    FourOfKind = 6,
    FiveOfKind = 7
}

impl HandType {
    pub fn from_cards (cards: &Vec<CARD>) -> Self {
        let mut counts = cards.into_iter().counts();
        let joker_count = counts.remove(&1).unwrap_or(0);
        let max_value = counts.values().max().unwrap_or(&0) + joker_count;

        let length = counts.len();

        match max_value {
            5 => Self::FiveOfKind,
            4 => Self::FourOfKind,
            3 => {
                if length == 2 {
                    Self::FullHouse
                } else {
                    Self::ThreeOfKind
                }
            },
            2 => {
                if length == 3 {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            },
            _ => Self::HighCard
        }
    }
}

// { joker: 1 }

type CARD = u32;

fn card_to_number(card: &str, has_joker: bool) -> CARD {
    match card {
        "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => card.parse().unwrap(),
        "T" => 10,

        // Jokers
        "J" => if has_joker { 1 } else { 11 },

        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => 0
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<CARD>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    pub fn from_str (line: &str, has_joker: bool) -> Self {
        let mut iter = line.split_whitespace();
        let cards: Vec<CARD> = iter
            .next()
            .unwrap()
            .chars()
            .map(|c|
                card_to_number(&c.to_string(), has_joker)
            ).collect();

        let bid: u32  = iter.next().unwrap().parse().unwrap();
        let hand_type = HandType::from_cards(&cards);

        Hand {
            cards,
            bid,
            hand_type
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type > other.hand_type {
            Ordering::Greater
        } else if self.hand_type < other.hand_type {
            Ordering::Less
        } else {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if self_card > other_card {
                    return Ordering::Greater;
                } else if self_card < other_card {
                    return Ordering::Less;
                }
            }

            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::from_str(line, false)).collect();

    hands.sort();

    let r: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| -> u32 {
            let order: u32 = 1 + u32::try_from(i).unwrap();
            dbg!(order, hand);

            order * hand.bid
        })
        .sum::<u32>();

    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::from_str(line, true)).collect();

    hands.sort();

    let r: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| -> u32 {
            let order: u32 = 1 + u32::try_from(i).unwrap();
            dbg!(order, hand);

            order * hand.bid
        })
        .sum::<u32>();

    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

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
