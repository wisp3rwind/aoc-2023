use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::hash_map::{OccupiedEntry, VacantEntry, Entry};
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
    fs::read_to_string(path).map_err(|source| AOCError::IOError {
        source,
        path: Some(path.into()),
    })
}

struct Data {
    path: String,
    network: HashMap<String, (String, String)>,
}

fn read_part1(input: &str) -> AOCResult<Data> {
    let mut lines = input.lines();

    let path = lines
        .next()
        .expect("input truncated, path missing")
        .to_owned();

    let network = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (from, to) = l.split_once('=').unwrap();
            let (to_left, to_right) = to
                .trim()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_once(',')
                .unwrap();

            (
                from.trim().to_owned(),
                (to_left.trim().to_owned(), to_right.trim().to_owned()),
            )
        })
        .collect();

    Ok(Data { path, network })
}

fn part1(data: &Data) -> AOCResult<usize> {
    let mut loc = "AAA";
    let mut steps = 0;
    let mut dirs = data.path.chars().cycle();
    while loc != "ZZZ" {
        let (next_left, next_right) = data.network.get(loc).expect("incomplete network map");
        loc = match dirs.next() {
            Some('L') => next_left,
            Some('R') => next_right,
            _ => panic!("Invalid path"),
        };
        steps += 1;
    }
    Ok(steps)
}

fn part2_brute_force(data: &Data) -> AOCResult<i64> {
    let mut locs: Vec<_> = data
        .network
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect();
    let mut steps = 0;
    let mut dirs = data.path.chars().cycle();
    //dbg!(&locs);
    while locs.iter().any(|node| !node.ends_with('Z')) {
        let dir = dirs.next();
        locs.iter_mut().for_each(|loc| {
            let (next_left, next_right) = data.network.get(*loc).expect("incomplete network map");
            *loc = match dir {
                Some('L') => next_left,
                Some('R') => next_right,
                _ => panic!("Invalid path"),
            };
        });
        //dbg!(&locs);
        steps += 1;
        if steps > 1_000_000_000 {
            panic!("infinite loop");
        }
    }
    Ok(steps)
}

fn part2(data: &Data) -> AOCResult<i64> {
    //let steps = Vec::<i64>::new();
    dbg!(data.path.len());
    for start in data.network.keys().filter(|node| node.ends_with('A')) {
        let mut loc = start;

        // last encounter of each loc
        let mut history: HashMap<String, usize> = Default::default();

        let mut cycle_start = 0;
        let mut cycle_len = 0;
        let mut step = 0;
        loop {
            match history.entry(loc.to_owned()) {
                Entry::Occupied(prev_encounter) => {
                    let prev_encounter = *prev_encounter.get();
                    cycle_start = prev_encounter;
                    cycle_len = step - prev_encounter;
                    dbg!(&history, loc);
                    break;
                },
                Entry::Vacant(new) => { new.insert(step); }
            };

            for dir in data.path.chars() {
                let (next_left, next_right) = data.network.get(loc).expect("incomplete network map");
                loc = match dir {
                    'L' => next_left,
                    'R' => next_right,
                    _ => panic!("Invalid path"),
                };
                dbg!(loc);
                step += 1;
            }

            if step > 100_000 {
                panic!("stuck");
            }
        }

        dbg!(start, cycle_start, cycle_len);
    }

    Ok(-1)
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

    aoc_test!(part11, "data/test1.txt", read_part1, super::part1, 2);
    aoc_test!(part12, "data/test2.txt", read_part1, super::part1, 6);
    aoc_test!(part2, "data/test3.txt", read_part1, super::part2, 6);
}
