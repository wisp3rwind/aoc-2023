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

impl Hand {
    fn typ(&self) -> HandType {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        self.hand.iter().for_each(|c| { counts.insert(*c, 0); });
        self.hand.iter().for_each(|c| {
            let count = counts.get_mut(c).unwrap();
            *count += 1;
        });

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

        // same type => lexicographic sort
        //for (cs, co) in self.hand.iter().zip(other.hand.iter()) {
            //if cs < co {
                //return Some(Ordering::Less);
            //} else if cs > co {
                //return Some(Ordering::Less);
            //}
        //}

        //return Some(Ordering::Equal);
        
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for Hand {
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

fn part2(data: &[Hand]) -> AOCResult<i64> {
    Err(AOCError::NotYetSolved)
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

    println!("Part 2: {}", part2(&data1)?);

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
    aoc_test!(part2, "data/test1.txt", read_part1, super::part2, 5905);
}
