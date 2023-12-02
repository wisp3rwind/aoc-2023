use std::borrow::Cow;
use std::fs;
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
    let mut input_file = std::env::current_dir()
        .map_err(|e| AOCError::IOError{source: e, path: None})?;
    input_file.push("dayXX");
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
            Ok(result) => assert_eq!(result, 0),
        }

        Ok(())
    }

    #[test]
    fn part2() -> AOCResult<()> {
        let path = "data/test2.txt";
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
