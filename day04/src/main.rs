use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
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

#[derive(Clone, Debug)]
struct Card {
    winning: HashSet<u8>,
    yours: Vec<u8>,
}

impl Card {
    fn num_matching(&self) -> usize {
        self.yours
            .iter()
            .filter(|num| self.winning.contains(num))
            .count()
    }

    fn score(&self) -> i64 {
        let count = self.num_matching();

        match count {
            0 => 0,
            _ => 2i64.pow(count as u32 - 1),
        }
    }
}

#[derive(Clone, Debug)]
struct Data {
    cards: Vec<Card>,
}

impl FromStr for Data {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cards = input
            .lines()
            .map(|l| {
                let (winning, yours) = l.split_once(':').unwrap().1.split_once('|').unwrap();
                let winning = winning
                    .split_ascii_whitespace()
                    .map(|w| w.parse::<u8>().unwrap())
                    .collect();
                let yours = yours
                    .split_ascii_whitespace()
                    .map(|w| w.parse::<u8>().unwrap())
                    .collect();
                Card { winning, yours }
            })
            .collect();

        Ok(Data { cards })
    }
}

trait FromFile<D: FromStr<Err = AOCError>> {
    fn from_file(path: impl AsRef<Path>) -> AOCResult<D> {
        let path = path.as_ref();
        fs::read_to_string(path)
            .map_err(|source| AOCError::IOError {
                source,
                path: Some(path.into()),
            })?
            .parse::<D>()
    }
}

impl<D: FromStr<Err = AOCError>> FromFile<D> for D {}

fn part1(data: &Data) -> AOCResult<(i64, Vec<i64>)> {
    let scores: Vec<_> = data.cards.iter().map(Card::score).collect();

    Ok((scores.iter().sum(), scores))
}

fn part2(data: &Data) -> AOCResult<i64> {
    let mut count = vec![1; data.cards.len()];

    for (i, card) in data.cards.iter().enumerate() {
        let ci = count[i];
        for j in (i + 1)..=(i + card.num_matching()) {
            if let Some(cj) = count.get_mut(j) {
                *cj += ci;
            }
        }
    }

    Ok(count.iter().sum::<usize>() as i64)
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day04");
    input_file.push("data");
    input_file.push("input.txt");

    let data = Data::from_file(input_file)?;
    println!("Part 1: {:?}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! aoc_test {
        (
            $func:ident,
            $datapath:literal,
            $dtype:ty,
            $compute:path,
            $expected:expr
            $(,)?
        ) => {
            #[test]
            fn $func() -> AOCResult<()> {
                match $compute(&<$dtype>::from_file($datapath)?) {
                    Ok(result) => assert_eq!(result, $expected),
                    Err(AOCError::NotYetSolved) => {}
                    Err(e) => return Err(e),
                };

                Ok(())
            }
        };
    }

    aoc_test!(
        part1,
        "data/test1.txt",
        Data,
        super::part1,
        (13, vec![8, 2, 2, 1, 0, 0]),
    );
    aoc_test!(part2, "data/test1.txt", Data, super::part2, 30);
}
