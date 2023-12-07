use std::borrow::Cow;
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
struct Data {
    races: Vec<(u64, u64)>,
}

impl FromStr for Data {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();

        let times = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(u64::from_str)
            .collect::<Result<Vec<_>, _>>().unwrap();
        let distances = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(u64::from_str)
            .collect::<Result<Vec<_>, _>>().unwrap();

        let races = times.iter().copied().zip(distances).collect();

        Ok(Data { races })
    }
}

fn read_part2(input: &str) -> AOCResult<(u64, u64)> {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    Ok((time, distance))
}

fn load_input(path: impl AsRef<Path>) -> AOCResult<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .map_err(|source| AOCError::IOError {
            source,
            path: Some(path.into()),
        })
}

fn part1(data: &Data) -> AOCResult<(u64, Vec<u64>)> {
    let mut winning_combos = Vec::new();
    for (time, distance) in &data.races {
        let wins = (0..=*time)
            .map(|charge| (*time - charge) * charge)
            .filter(|dist| dist > distance)
            .count() as u64;
        winning_combos.push(wins);
    }

    let total = winning_combos.iter().product();

    Ok((total, winning_combos))
}

fn part2(input: &(u64, u64)) -> AOCResult<u64> {
    let (time, distance) = dbg!(*input);

    // solve (t - c) c == dist
    // => c^2 - 2 (t / 2) c == -dist
    // => (c - t / 2)^2 - t^2 / 4 == -dist

    let t = time as f64;
    let d = distance as f64;
    let x = (0.25 * t * t - d).sqrt();
    let c1 = (0.5 * t - x).ceil() as u64;
    let c2 = (0.5 * t + x).floor() as u64;

    let t = time as u64;
    dbg!(c1 > 0);
    dbg!(c2 < t);
    assert!((t - c1) * c1 > distance);
    assert!((t - (c1 - 1)) * (c1 - 1) < distance);
    assert!((t - c2) * c2 > distance);
    assert!((t - (c2 + 1)) * (c2 + 1) < distance);

    Ok(c2 - c1 + 1)
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day06");
    input_file.push("data");
    input_file.push("input.txt");

    let input = load_input(&input_file)?;

    let data1 = Data::from_str(&input)?;
    println!("Part 1: {:?}", part1(&data1)?);

    let data2 = read_part2(&input)?;
    println!("Part 2: {}", part2(&data2)?);

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
                match $compute(&$read_data(&input)?) {
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
        Data::from_str,
        super::part1,
        (288, vec![4, 8, 9])
    );
    aoc_test!(part2, "data/test1.txt", read_part2, super::part2, 71503);
}
