use itertools::Itertools;
use regex::Regex;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Index;
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
struct MapInterval {
    len: usize,
    src_start: usize,
    dest_start: usize,
}

impl FromStr for MapInterval {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((dest_start, src_start, len)) = s
            .split_ascii_whitespace()
            .map(usize::from_str)
            .collect_tuple() {
            Ok(Self {
                len: len.unwrap(),
                src_start: src_start.unwrap(),
                dest_start: dest_start.unwrap()
            })
        } else {
            Err(AOCError::ParseError { msg: "incorrect range".into() })
        }
    }
}

#[derive(Clone, Debug)]
struct AMap {
    ranges: Vec<MapInterval>,
}

impl AMap {
    fn get(&self, index: usize) -> usize {
        for MapInterval {len, src_start, dest_start} in &self.ranges {
            if index >= *src_start && index < *src_start + *len {
                return *dest_start + index - *src_start;
            }
        }

        index
    }

    fn get_range(&self, start: usize, len: usize) -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        let mut start = start;
        let mut remaining = len;
        let mut cur_len = 0;
        while remaining > 0 {
            //dbg!(start, remaining);
            let mut next = usize::MAX;
            for MapInterval {len, src_start, dest_start} in &self.ranges {
                if *src_start > start {
                    next = next.min(*src_start);
                }
                if start >= *src_start && start < *src_start + *len {
                    let offset = start - *src_start;
                    let cur_dest = *dest_start + offset;
                    cur_len = (len - offset).min(remaining);
                    //dbg!(cur_dest, cur_len);
                    out.push((cur_dest, cur_len));
                    break;
                }
            }

            if cur_len == 0 {
                cur_len = (next - start).min(remaining);
                //dbg!(start, cur_len);
                out.push((start, cur_len));
            }
            start = start + cur_len;
            remaining = remaining - cur_len;
            cur_len = 0;
        }

        assert_eq!(len, out.iter().map(|(_, l)| l).sum());

        out
    }
}

#[derive(Clone, Debug)]
struct Data {
    seeds: Vec<usize>,
    maps: HashMap<String, (String, AMap)>,
}

impl FromStr for Data {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();

        let seeds = lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_ascii_whitespace()
            .map(usize::from_str)
            .collect::<Result<_, _>>()
            .unwrap();

        let re = Regex::new("([^-]+)-to-([^-]+) map:").unwrap();

        let mut maps = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            if let Some(cap) = re.captures(line) {
                let mut map = AMap { ranges: Vec::new() };
                let from = cap[1].to_owned();
                let to = cap[2].to_owned();

                while let Some(line) = lines.next() {
                    let line = line.trim();
                    if line.is_empty() {
                        break;
                    }

                    map.ranges.push(line.parse().unwrap());
                }

                maps.insert(from, (to, map));
            } else {
                return Err(AOCError::ParseError{msg: "not a map".into()});
            }
        }

        Ok(Data { seeds, maps })
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

fn part1(data: &Data) -> AOCResult<(usize, HashSet<usize>)> {
    //dbg!(data);

    let mut locations = HashSet::new();
    for seed in &data.seeds {
        let mut id = *seed;
        let mut key = "seed";
        while key != "location" {
            let (dest, map) = &data.maps[key];
            key = dest;
            id = map.get(id);
        }
        locations.insert(id);
    }

    let closest = locations.iter().min().unwrap();
    Ok((*closest, locations))
}

fn part2(data: &Data) -> AOCResult<usize> {
    let mut locations = HashSet::new();

    let mut ranges: Vec<_> = data.seeds.iter().copied().tuples().collect();
    let mut key = "seed";
    while key != "location" {
        //dbg!(&ranges);
        let (dest, map) = &data.maps[key];
        key = dest;
        let mut new_ranges = Vec::new();
        for (start, len) in ranges.iter().copied() {
            new_ranges.append(
                &mut map.get_range(start, len)
            );
        }
        ranges = new_ranges;
    }
    //dbg!(&ranges);

    for (start, _) in ranges.iter() {
        locations.insert(start);
    }

    dbg!(&locations);


    Ok(
        *locations
        .iter()
        .copied()
        .min()
        .unwrap()
    )
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day05");
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
            $(,)?  // allow (optional) trailing comma
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
        (35, HashSet::from([82, 43, 86, 35]))
    );
    aoc_test!(part2, "data/test1.txt", Data, super::part2, 46);
}
