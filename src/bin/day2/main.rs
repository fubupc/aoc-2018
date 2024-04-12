use anyhow::{anyhow, Ok, Result};
use std::{collections::HashMap, env};

use aoc_2018::parse_file;

fn id_list_checksum(ids: &[String]) -> u64 {
    let mut accum_2_times = 0;
    let mut accum_3_times = 0;
    for id in ids.iter() {
        let (appear_2_times, appear_3_times) = appear_2_or_3_times(id);
        if appear_2_times {
            accum_2_times += 1;
        }
        if appear_3_times {
            accum_3_times += 1;
        }
    }
    accum_2_times * accum_3_times
}

fn appear_2_or_3_times(id: &str) -> (bool, bool) {
    let mut count = HashMap::new();
    for c in id.chars() {
        count.entry(c).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let appear_2_times = count.iter().any(|(_, &cnt)| cnt == 2);
    let appear_3_times = count.iter().any(|(_, &cnt)| cnt == 3);

    (appear_2_times, appear_3_times)
}

fn diff_by_1_char(id1: &str, id2: &str) -> Option<usize> {
    let mut id1_iter = id1.char_indices();
    let mut id2_iter = id2.char_indices();
    let mut position = None;

    loop {
        match (id1_iter.next(), id2_iter.next()) {
            (Some((index, c1)), Some((_, c2))) => {
                if c1 != c2 {
                    if position.is_some() {
                        return None;
                    }
                    position = Some(index);
                }
            }
            (None, None) => return position,
            _ => return None, // length mismatch
        }
    }
}

fn common_of_correct_ids(ids: &[String]) -> String {
    for i in 0..ids.len() {
        for j in i..ids.len() {
            if let Some(pos) = diff_by_1_char(&ids[i], &ids[j]) {
                let mut copy = ids[i].clone();
                copy.remove(pos);
                return copy;
            }
        }
    }
    panic!("not found")
}

fn main() -> Result<()> {
    let mut args = env::args();
    let _ = args.next().ok_or(anyhow!("arg 0 ??"));
    let input = args.next().ok_or(anyhow!("need input file"))?;

    let ids: Vec<String> = parse_file(input)?;
    println!("Part One: {}", id_list_checksum(&ids));
    println!("Part Two: {}", common_of_correct_ids(&ids));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::diff_by_1_char;

    #[test]
    fn test_diff_by_1_char() {
        assert_eq!(diff_by_1_char("fghij", "fguij"), Some(2));
    }
}
