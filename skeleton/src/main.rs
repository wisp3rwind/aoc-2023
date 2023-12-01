use std::borrow::Cow;
use std::fs;
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
enum AOCError {
    #[error("Failed to read input")]
    IOError,

    #[error("Failed to parse input {msg}")]
    ParseError { msg: Cow<'static, str> },

    #[error("This part of the puzzle is not yet implemented")]
    NotYetSolved,
}

type AOCResult<T> = Result<T, AOCError>;

#[derive(Clone, Debug)]
struct Data {
    items: Vec<String>,
    //items: Vec<u64>,
}

impl FromStr for Data {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let items = input
            .lines()
            .map(|l| l.to_owned())
            .collect();
            //.map(|l| l.parse::<u64>())
            //.collect::<Result<_, _>>()
            //.map_err(|_e| AOCError::ParseError { msg: "...".into() })?;
        
        Ok(Data { items })
    }
}

fn part1 (data: &Data) -> AOCResult<i64> {
    Err(AOCError::NotYetSolved)
}

fn part2 (data: &Data) -> AOCResult<i64> {
    Err(AOCError::NotYetSolved)
}

fn main() -> AOCResult<()> {
    let data = fs::read_to_string("data/input.txt")
            .map_err(|_e| AOCError::IOError)?
            .parse::<Data>()?;

    println!("Part 1: {}", part1(&data)?);
    println!("Part 2: {}", part2(&data)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() -> AOCResult<()> {
        let data = fs::read_to_string("data/test.txt")
                .map_err(|_e| AOCError::IOError)?
                .parse::<Data>()?;

gg      match part1(&data) {
            Err(AOCError::NotYetSolved) => {},
            Err(_e) => {
                assert!(false)
            },
            Ok(result) => assert_eq!(result, 0),
        }

        match part2(&data) {
            Err(AOCError::NotYetSolved) => {},
            Err(_e) => {
                assert!(false)
            },
            Ok(result) => assert_eq!(result, 0),
        }

        Ok(())
    }
}
