use regex::Regex;
use std::borrow::Cow;
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

#[derive(Clone, Debug)]
struct Data1 {
    items: Vec<(u8, Option<u8>)>,
}

impl FromStr for Data1 {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let items: AOCResult<Vec<_>> = input
            .lines()
            .map(|l| {
                let mut it = l.chars();

                let first = it
                    .find(|c| c.is_ascii_digit())
                    .ok_or(AOCError::ParseError {
                        msg: "No digit in input line".into(),
                    })?;

                let last = it.rfind(|c| c.is_ascii_digit());

                Ok((
                    first.to_digit(10).unwrap() as u8,
                    last.map(|c| c.to_digit(10).unwrap() as u8),
                ))
            })
            .collect();

        Ok(Data1 { items: items? })
    }
}

#[derive(Clone, Debug)]
struct Data2 {
    items: Vec<(u8, u8)>,
}

fn parse_digit(s: &str) -> AOCResult<u8> {
    let digit = match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        digit => digit.chars().next().unwrap().to_digit(10).unwrap() as u8,
    };

    Ok(digit)
}

impl FromStr for Data2 {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let re_rev = Regex::new("([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

        let items: AOCResult<Vec<_>> = input
            .lines()
            .map(|l| {
                let first = re.find_iter(l).next().ok_or(AOCError::ParseError {
                    msg: "No digit in input line".into(),
                })?;

                // find_iter() only returns non-overlapping matches, so we
                // can't use the above iterator's last() to obtain the last
                // digit, since the input can (and does) contain cases like
                // "twone"
                let l_rev = l.chars().rev().collect::<String>();
                let last = re_rev
                    .find_iter(&l_rev)
                    .next()
                    .ok_or(AOCError::ParseError {
                        msg: "No digit in input line".into(),
                    })?
                    .as_str()
                    .chars()
                    .rev()
                    .collect::<String>();

                Ok((parse_digit(first.as_str())?, parse_digit(last.as_str())?))
            })
            .collect();

        Ok(Data2 { items: items? })
    }
}

fn part1(data: &Data1) -> AOCResult<u64> {
    let sum = data
        .items
        .iter()
        .copied()
        .map(|(first, last)| {
            (match last {
                Some(last) => first * 10 + last,
                None => 11 * first,
            }) as u64
        })
        .sum();
    Ok(sum)
}

fn part2(data: &Data2) -> AOCResult<u64> {
    let sum = data
        .items
        .iter()
        .copied()
        .map(|(first, last)| (first * 10 + last) as u64)
        .sum();
    Ok(sum)
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day01");
    input_file.push("data");
    input_file.push("input.txt");

    let raw_data = fs::read_to_string(&input_file).map_err(move |source| AOCError::IOError {
        source,
        path: Some(input_file),
    })?;

    let data = raw_data.parse::<Data1>()?;
    println!("Part 1: {}", part1(&data)?);

    let data = raw_data.parse::<Data2>()?;
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
            .map_err(|source| AOCError::IOError {
                source,
                path: Some(path.into()),
            })?
            .parse::<Data1>()?;

        match super::part1(&data) {
            Err(AOCError::NotYetSolved) => {}
            Err(_e) => {
                assert!(false)
            }
            Ok(result) => assert_eq!(result, 142),
        }

        Ok(())
    }

    #[test]
    fn part2() -> AOCResult<()> {
        let path = "data/test2.txt";
        let data = fs::read_to_string(path)
            .map_err(|source| AOCError::IOError {
                source,
                path: Some(path.into()),
            })?
            .parse::<Data2>()?;

        match super::part2(&data) {
            Err(AOCError::NotYetSolved) => {}
            Err(_e) => {
                assert!(false)
            }
            Ok(result) => assert_eq!(result, 281),
        }

        Ok(())
    }
}
