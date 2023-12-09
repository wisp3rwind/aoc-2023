use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
enum AOCError {
    #[error("Failed to read input: {path:?}")]
    IOError {
        source: std::io::Error,
        path: Option<PathBuf>,
    },

    #[error("Failed to parse input {msg}")]
    #[allow(unused)]
    ParseError { msg: Cow<'static, str> },

    #[error("This part of the puzzle is not yet implemented")]
    #[allow(unused)]
    NotYetSolved,
}

type AOCResult<T> = Result<T, AOCError>;

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
enum HandType {
    FiveOfAKind = 10,
    FourOfAKind = 9,
    FullHouse = 8,
    ThreeOfAKind = 7,
    TwoPair = 6,
    OnePair = 5,
    HighCard = 4,
}

#[derive(Clone, Debug)]
struct Hand {
    bid: u32,
    hand: [u8; 5],
}

#[derive(Clone, Debug)]
struct HandWithJokers {
    bid: u32,
    hand: [u8; 5],
}


impl Hand {
    fn typ(&self) -> HandType {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        self.hand.iter().for_each(|c| { *counts.entry(*c).or_insert(0) += 1; });

        match counts.values().copied().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if let Some(_) = counts.values().find(|c| **c == 2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            },
            2 => {
                if counts.values().filter(|c| **c == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            },
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

// Could probably simplify this (i.e. re-use Hand.typ) by actually replacing
// J with the appropriate card (which should always be the most frequent one
// among the others)
impl HandWithJokers {
    fn typ(&self) -> HandType {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        self.hand.iter().for_each(|c| { *counts.entry(*c).or_insert(0) += 1; });

        let jack_count = counts.get(&1);
        match counts.values().copied().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => {
                match jack_count {
                    Some(4) => HandType::FiveOfAKind,
                    Some(1) => HandType::FiveOfAKind,
                    _ => HandType::FourOfAKind,
                }
            },
            3 => {
                if let Some(_) = counts.values().find(|c| **c == 2) {
                    match jack_count {
                        Some(3) => HandType::FiveOfAKind,  // 3 J + 1 pair
                        Some(2) => HandType::FiveOfAKind,  // 2 J + triplett
                        None => HandType::FullHouse,  // no j, but 2 + 3
                        _ => unreachable!(),
                    }
                } else {
                    match jack_count {
                        Some(3) => HandType::FourOfAKind,  // triplett of J + 2 single
                        Some(1) => HandType::FourOfAKind, // triplett + single J
                        None => HandType::ThreeOfAKind,  // triplett + 2 single
                        _ => unreachable!(),
                    }
                }
            },
            2 => {
                if counts.values().filter(|c| **c == 2).count() == 2 {
                    match jack_count {
                        Some(2) => HandType::FourOfAKind,  // 2 pairs, one of which J
                        Some(1) => HandType::FullHouse,  // 2 pairs + 1 J
                        None => HandType::TwoPair,  // just 2 pairs
                        _ => unreachable!(),
                    }
                } else {
                    match jack_count {
                        Some(2) => HandType::ThreeOfAKind,  // 1 pair of J, 3 single
                        Some(1) => HandType::ThreeOfAKind,  // 1 pair, 1 J, 2 other single
                        None => HandType::OnePair,  // 1 pair, 3 single
                        _ => unreachable!(),
                    }
                }
            },
            1 => {
                match jack_count {
                    Some(1) => HandType::OnePair,  // singles, 1 of which J
                    None => HandType::HighCard,  // single cards only
                    _ => unreachable!(),
                }
            },
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Hand { }

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ts = self.typ();
        let to = other.typ();
        if ts < to {
            return Some(Ordering::Less);
        } else if ts > to {
            return Some(Ordering::Greater);
        }

        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for HandWithJokers {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for HandWithJokers { }

impl PartialOrd for HandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ts = self.typ();
        let to = other.typ();
        if ts < to {
            return Some(Ordering::Less);
        } else if ts > to {
            return Some(Ordering::Greater);
        }

        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for HandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn read_part1(input: &str) -> AOCResult<Vec<Hand>> {
    Ok(input.lines()
        .map(|l| {
            let (hand_str, bid) = l.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let mut hand = [0u8; 5];
            for (i, c) in hand_str.chars().enumerate() {
                let c = match c {
                    '2'..='9' => (c as u8 - '2' as u8) as u8 + 2,
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("invalid card"),
                };
                hand[i] = c;
            }

            Hand { bid, hand }
        })
        .collect())
}

fn read_part2(input: &str) -> AOCResult<Vec<HandWithJokers>> {
    Ok(input.lines()
        .map(|l| {
            let (hand_str, bid) = l.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let mut hand = [0u8; 5];
            for (i, c) in hand_str.chars().enumerate() {
                let c = match c {
                    '2'..='9' => (c as u8 - '2' as u8) as u8 + 2,
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("invalid card"),
                };
                hand[i] = c;
            }

            HandWithJokers { bid, hand }
        })
        .collect())
}

fn load_input(path: impl AsRef<Path>) -> AOCResult<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|source| AOCError::IOError {
            source,
            path: Some(path.into()),
        })
}

fn part1(data: &mut [Hand]) -> AOCResult<u64> {
    data.sort_unstable();

    Ok(data.iter().enumerate().map(|(rank, hand)| {
        //dbg!(rank, hand.bid);
        (hand.bid as u64) * (rank as u64 + 1)
    }).sum::<u64>())
}

fn part2(data: &mut [HandWithJokers]) -> AOCResult<u64> {
    data.sort_unstable();
    //dbg!(&data);

    Ok(data.iter().enumerate().map(|(rank, hand)| {
        (hand.bid as u64) * (rank as u64 + 1)
    }).sum::<u64>())
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day07");
    input_file.push("data");
    input_file.push("input.txt");

    let input = load_input(&input_file)?;

    let mut data1 = read_part1(&input)?;
    println!("Part 1: {:?}", part1(&mut data1)?);

    let mut data2 = read_part2(&input)?;
    println!("Part 2: {}", part2(&mut data2)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! aoc_test {
        (
            $func:ident,
            $datapath:literal,
            $read_data:path,
            $compute:path,
            $expected:expr
            $(,)?  // allow (optional) trailing comma
        ) => {
            #[test]
            fn $func() -> AOCResult<()> {
                let input = load_input($datapath)?;
                match $compute(&mut $read_data(&input)?) {
                    Ok(result) => assert_eq!(result, $expected),
                    Err(AOCError::NotYetSolved) => {}
                    Err(e) => return Err(e),
                };

                Ok(())
            }
        };
    }

    aoc_test!(part1, "data/test1.txt", read_part1, super::part1, 6440);
    aoc_test!(part2, "data/test1.txt", read_part2, super::part2, 5905);
}
