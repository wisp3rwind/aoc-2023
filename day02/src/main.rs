use std::borrow::Cow;
use std::cmp::{Ordering, PartialOrd};
use std::fs;
use std::path::PathBuf;
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
        if self.red < other.red && self.red < other.red && self.red < other.red
            { Some(Ordering::Less) }
        else if self.red == other.red&& self.red == other.red&& self.red == other.red
            { Some(Ordering::Equal) } 
        else if self.red < other.red&& self.red < other.red&& self.red < other.red
            { Some(Ordering::Less) }
        else
            { None }

    }
}

impl FromStr for Draw {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut out = Draw::default();

        for s in input.split(",") {
            let (count, color) = dbg!(s.trim().split_once(" ").unwrap());
            let count = count.trim().parse::<usize>().unwrap();
            match color.trim() {
                "red" => { out.red += count },
                "green" => { out.green += count },
                "blue" => { out.blue += count },
                _ => {
                    return Err(AOCError::ParseError { msg: "unknown color".into() })
                }
            };
        }

        Ok(out)
    }
}

#[derive(Clone, Debug)]
struct Data {
    games: Vec<Vec<Draw>>,
}

impl FromStr for Data {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let games = input
            .lines()
            .map(|l| {
                l.split_once(":").unwrap()
                    .1
                    .split(";")
                    .map(Draw::from_str)
                    .collect::<AOCResult<_>>()
            })
            .collect::<AOCResult<_>>()?;
            //.map(|l| l.parse::<u64>())
            //.collect::<Result<_, _>>()
            //.map_err(|_e| AOCError::ParseError { msg: "...".into() })?;
        
        Ok(Data { games })
    }
}

fn part1 (data: &Data) -> AOCResult<usize> {
    let total = Draw { red: 12, green: 13, blue: 14 };
    let sum = data.games.iter()
        .enumerate()
        .map(|(id, draws)| 
            if draws.iter().copied().all(|d| d <= total) { id + 1 } else { 0 }
        )
        .sum();
    Ok(sum)
}

fn part2 (data: &Data) -> AOCResult<i64> {
    Err(AOCError::NotYetSolved)
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir()
        .map_err(|e| AOCError::IOError{source: e, path: None})?;
    input_file.push("day02");
    input_file.push("data");
    input_file.push("input.txt");

    let raw_data = fs::read_to_string(&input_file)
            .map_err(move |source| AOCError::IOError{source, path: Some(input_file)})?;

    let data = raw_data.parse::<Data>()?;
    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() -> AOCResult<()> {
        let path = "data/test1.txt";
        let data = fs::read_to_string(path)
                .map_err(|source| AOCError::IOError{source, path: Some(path.into())})?
                .parse::<Data>()?;

        match super::part1(&data) {
            Err(AOCError::NotYetSolved) => {},
            Err(_e) => {
                assert!(false)
            },
            Ok(result) => assert_eq!(result, 8),
        }

        Ok(())
    }

    #[test]
    fn part2() -> AOCResult<()> {
        let path = "data/test1.txt";
        let data = fs::read_to_string(path)
                .map_err(|source| AOCError::IOError{source, path: Some(path.into())})?
                .parse::<Data>()?;

        match super::part2(&data) {
            Err(AOCError::NotYetSolved) => {},
            Err(_e) => {
                assert!(false)
            },
            Ok(result) => assert_eq!(result, 0),
        }

        Ok(())
    }
}
