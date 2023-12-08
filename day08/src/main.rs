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

fn read_part1(input: &str) -> AOCResult<Vec<String>> {
    Ok(input.lines()
        .map(str::to_owned)
        .collect()
    )
}

fn part1(data: &Vec<String>) -> AOCResult<i64> {
    Err(AOCError::NotYetSolved)
}

fn part2(data: &Vec<String>) -> AOCResult<i64> {
    Err(AOCError::NotYetSolved)
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day08");
    input_file.push("data");
    input_file.push("input.txt");

    let input = load_input(&input_file)?;

    let data1 = read_part1(&input)?;
    println!("Part 1: {:?}", part1(&data1)?);

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

    aoc_test!(part1, "data/test1.txt", read_part1, super::part1, 0);
    aoc_test!(part2, "data/test1.txt", read_part1, super::part2, 0);
}
