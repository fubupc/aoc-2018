use anyhow::{anyhow, Ok, Result};
use std::{collections::HashSet, env};

use aoc_2018::parse_file;

fn calibrate_freq(changes: &[i32]) -> i32 {
    changes.iter().fold(0, |acc, x| acc + x)
}

fn first_freq_reach_twice(changes: &[i32]) -> i32 {
    let mut freq = 0;
    let mut reached = HashSet::from([freq]);

    loop {
        for &c in changes {
            freq += c;
            if reached.contains(&freq) {
                return freq;
            }
            reached.insert(freq);
        }
    }
}

fn main() -> Result<()> {
    let mut args = env::args();
    let _ = args.next().ok_or(anyhow!("arg 0 ??"));
    let input = args.next().ok_or(anyhow!("need input file"))?;

    let changes: Vec<i32> = parse_file(input)?;
    println!("Part One: {}", calibrate_freq(&changes));
    println!("Part Two: {}", first_freq_reach_twice(&changes));

    Ok(())
}
