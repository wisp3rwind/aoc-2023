use itertools::Itertools;
use std::borrow::Cow;
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

fn load_input(path: impl AsRef<Path>) -> AOCResult<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|source| AOCError::IOError {
            source,
            path: Some(path.into()),
        })
}

fn read_part1(input: &str) -> AOCResult<Vec<Vec<i64>>> {
    Ok(input.lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
    )
}

fn finite_diff_extrapolation(data: &[i64]) -> (i64, i64) {
    if data.iter().all_equal() {
        let diff = *data.iter().next().unwrap();
        (diff, diff)
    } else {
        let differences: Vec<_> = data.iter().copied()
            .tuple_windows()
            .map(|(x1, x2)| x2 - x1)
            .collect();
        let (diff_front, diff_back) = finite_diff_extrapolation(&differences);
        let front = data.iter().next().unwrap() - diff_front;
        let back = data.iter().rev().next().unwrap() + diff_back;
        (front, back)
    }
}

fn part1(data: &Vec<Vec<i64>>) -> AOCResult<(i64, Vec<i64>)> {
    let mut extrapolations = Vec::new();

    for x in data {
        extrapolations.push(finite_diff_extrapolation(&x).1);
    }

    let total = extrapolations.iter().sum();
    Ok((total, extrapolations))
}

fn part2(data: &Vec<Vec<i64>>) -> AOCResult<(i64, Vec<i64>)> {
    let mut extrapolations = Vec::new();

    for x in data {
        extrapolations.push(finite_diff_extrapolation(&x).0);
    }

    let total = extrapolations.iter().sum();
    Ok((total, extrapolations))
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day09");
    input_file.push("data");
    input_file.push("input.txt");

    let input = load_input(&input_file)?;

    let data1 = read_part1(&input)?;
    println!("Part 1: {:?}", part1(&data1)?);

    println!("Part 2: {:?}", part2(&data1)?);

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

    aoc_test!(part1, "data/test1.txt", read_part1, super::part1, (114, vec![18, 28, 68]));
    aoc_test!(part2, "data/test1.txt", read_part1, super::part2, (2, vec![-3, 0, 5]));
}
