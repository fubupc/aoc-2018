use anyhow::{anyhow, Error, Result};
use std::{io::BufRead, path::Path, str::FromStr};

pub fn parse_file<P, T, E>(path: P) -> Result<Vec<T>, Error>
where
    P: AsRef<Path>,
    E: std::fmt::Display,
    T: FromStr<Err = E>,
{
    let f = std::fs::File::open(path)?;

    std::io::BufReader::new(f)
        .lines()
        .map(|l| l?.parse::<T>().map_err(|e| anyhow!("{}", e)))
        .collect()
}
