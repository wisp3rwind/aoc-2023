use std::borrow::Cow;
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
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
    ParseError { msg: Cow<'static, str> },

    #[error("This part of the puzzle is not yet implemented")]
    #[allow(unused)]
    NotYetSolved,
}

type AOCResult<T> = Result<T, AOCError>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.red == other.red && self.green == other.green && self.blue == other.blue {
            Some(Ordering::Equal)
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(Ordering::Less)
        } else if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Draw {
    fn contains_all<'a>(self, others: impl IntoIterator<Item = &'a Self>) -> bool {
        others.into_iter().copied().all(|d| d <= self)
    }

    fn union(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(self) -> usize {
        self.red * self.blue * self.green
    }
}

impl FromStr for Draw {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut out = Draw::default();

        for s in input.split(',') {
            let (count, color) = s.trim().split_once(' ').unwrap();
            let count = count.trim().parse::<usize>().unwrap();
            match color.trim() {
                "red" => out.red += count,
                "green" => out.green += count,
                "blue" => out.blue += count,
                _ => {
                    return Err(AOCError::ParseError {
                        msg: "unknown color".into(),
                    })
                }
            };
        }

        Ok(out)
    }
}

#[derive(Clone, Debug)]
struct Data {
    games: HashMap<usize, Vec<Draw>>,
}

impl FromStr for Data {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let games = input
            .lines()
            .map(|l| {
                let (id, draws) = l.split_once(':').unwrap();
                let id = id
                    .strip_prefix("Game")
                    .unwrap()
                    .trim()
                    .parse::<usize>()
                    .unwrap();
                let draws = draws
                    .split(';')
                    .map(Draw::from_str)
                    .collect::<AOCResult<_>>();
                match draws {
                    Ok(draws) => Ok((id, draws)),
                    Err(e) => Err(e),
                }
            })
            .collect::<AOCResult<_>>()?;

        Ok(Data { games })
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

fn part1(data: &Data) -> AOCResult<usize> {
    let total = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum = data
        .games
        .iter()
        .map(|(&id, draws)| if total.contains_all(draws) { id } else { 0 })
        .sum();
    Ok(sum)
}

fn part2(data: &Data) -> AOCResult<usize> {
    let total = data
        .games
        .values()
        .map(|draws| draws.iter().copied().reduce(Draw::union).unwrap())
        .map(Draw::power)
        .sum();

    Ok(total)
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day02");
    input_file.push("data");
    input_file.push("input.txt");

    let data = Data::from_file(input_file)?;
    println!("Part 1: {}", part1(&data)?);
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
            $expected:literal
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

    aoc_test!(part1, "data/test1.txt", Data, super::part1, 8);
    aoc_test!(part2, "data/test1.txt", Data, super::part2, 2286);
}
