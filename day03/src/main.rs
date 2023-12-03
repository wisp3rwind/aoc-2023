use std::borrow::Cow;
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

// FIXME: Didn't really turn out to be a very useful datastructure: Due to
// duplicating the ids in id_map, I need to constantly pay attention to dedup
// again when doing the actual computation.
// In principle, this code should have linear scaling (with the number of parts),
// but it would be nicer to abstract it away into a generic data structure that
// handles the duplication issues.
#[derive(Clone, Debug)]
struct Data {
    // (id, is_part)
    ids: Vec<(u32, bool)>,

    // (x, y) -> entry in ids
    id_map: HashMap<(i32, i32), usize>,

    // (x, y) -> part
    parts: HashMap<(i32, i32), char>,
}

impl FromStr for Data {
    type Err = AOCError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut ids = Vec::new();
        let mut id_map = HashMap::new();
        let mut parts = HashMap::new();

        let mut chars = Vec::new();

        let mut store_id = |x: i32, y: i32, chars: &mut Vec<char>| -> AOCResult<()> {
            let num_digits = chars.len() as i32;
            if num_digits == 0 {
                return Ok(());
            }
            let id: String = chars.drain(..).collect();
            let id = id.parse::<u32>()
                .map_err(
                    |_| {
                        AOCError::ParseError { msg: format!("not an integer '{}'", &id).into() }
                    }
                )?;
            ids.push((id, false));
            let idx = ids.len() - 1;
            for offset in 1..=num_digits {
                id_map.insert((x - offset, y), idx);
            }

            Ok(())
        };

        for (y, l) in input.lines().enumerate() {
            let y = y as i32;
            let mut x = 0;

            for (xu, c) in l.chars().enumerate() {
                x = xu as i32;

                match c {
                    '.' => {},
                    '0'..='9' => {
                        chars.push(c);
                        continue;
                    },
                    _ => { parts.insert((x, y), c); }
                }

                // A number ended, parse and store it
                store_id(x, y, &mut chars)?;
            }

            // Line ended, thus, number must also end
            store_id(x, y, &mut chars)?;
        }

        Ok(Data { ids, id_map, parts })
    }
}

trait FromFile<D: FromStr<Err = AOCError>> {
    fn from_file(path: impl AsRef<Path>) -> AOCResult<D> {
        let path = path.as_ref();
        Ok(fs::read_to_string(path)
            .map_err(|source| AOCError::IOError {
                source,
                path: Some(path.into()),
            })?
            .parse::<D>()?)
    }
}

impl<D: FromStr<Err = AOCError>> FromFile<D> for D {}

fn part1(data: &mut Data) -> AOCResult<u64> {
    for (x, y) in data.parts.keys() {
        for xi in (x - 1)..=(x + 1) {
            for yi in (y - 1)..=(y + 1) {
                if let Some(idx) = data.id_map.get_mut(&(xi, yi)) {
                    data.ids[*idx].1 = true;
                }
            }
        }
    }

    Ok(data.ids.iter().copied()
        .fold(0, |total, (id, is_part)| {
            if is_part { id as u64 + total } else { total }
        })
    )
}

fn part2(data: &Data) -> AOCResult<i32> {
    Ok(data.parts.iter()
        .filter_map(|(loc, c)| if *c == '*' { Some(loc) } else { None })
        .map(|(x, y)| {
            let mut ids = Vec::new();
            for xi in (x - 1)..=(x + 1) {
                for yi in (y - 1)..=(y + 1) {
                    if let Some(idx) = data.id_map.get(&(xi, yi)) {
                        ids.push(data.ids[*idx].0 as i32); 
                    }
                }
            }
            ids.sort();
            ids.dedup();
            if ids.len() == 2 { ids.drain(..).fold(1, |gear_ratio, id| gear_ratio * id) } else { 0 }
        })
        .sum())
}

fn main() -> AOCResult<()> {
    let mut input_file = std::env::current_dir().map_err(|e| AOCError::IOError {
        source: e,
        path: None,
    })?;
    input_file.push("day03");
    input_file.push("data");
    input_file.push("input.txt");

    let mut data = Data::from_file(input_file)?;
    println!("Part 1: {}", part1(&mut data)?);
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
                match $compute(&mut <$dtype>::from_file($datapath)?) {
                    Ok(result) => assert_eq!(result, $expected),
                    Err(AOCError::NotYetSolved) => {}
                    Err(e) => return Err(e),
                };

                Ok(())
            }
        };
    }

    aoc_test!(part1, "data/test1.txt", Data, super::part1, 4361);
    aoc_test!(part2, "data/test1.txt", Data, super::part2, 467835);
}
