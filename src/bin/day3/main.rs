use anyhow::{anyhow, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::BTreeMap, env, str::FromStr};

use aoc_2018::parse_file;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap());
        match RE.captures(s) {
            Some(caps) => {
                let (_, [id, left, top, width, height]) = caps.extract();
                let id = id.parse()?;
                let left = left.parse()?;
                let top = top.parse()?;
                let width: usize = width.parse()?;
                let height: usize = height.parse()?;

                Ok(Claim {
                    id,
                    left,
                    top,
                    right: left + width,
                    bottom: top + height,
                })
            }
            None => Err(anyhow!("invalid claim string: {}", s)),
        }
    }
}

// Divide the whole fabric into small grids by lines formed by the edges of claims, then iterate over the grids
// to check if any one appears in two or more claims.
// Assumptions:
// - X axis extends from left to right
// - Y axis extends from top to bottom
// - Top-left corner as the origin
fn fabric_overlap(claims: &[Claim]) -> usize {
    // Sort claims by their X coordinates, with each claim's left and right edges as two separate X coordinates.
    let mut x_sorted: Vec<_> = claims
        .iter()
        .flat_map(|c| [(c.left, c), (c.right, c)])
        .collect();
    x_sorted.sort_by_key(|&(x, _)| x);

    // Sort claims by their X coordinates, with each claim's top and bottom edges as two separate Y coordinates.
    let mut y_sorted: Vec<_> = claims
        .iter()
        .flat_map(|c| [(c.top, c), (c.bottom, c)])
        .collect();
    y_sorted.sort_by_key(|&(y, _)| y);

    // Total area of overlapping
    let mut total_overlap = 0;

    // Every two adjacent (distinct) X coordinates and two adjacent (distinct) Y coordinates define a grid.
    // NOTE: Multiple claims may share the same X/Y coordinate.
    let mut left_idx = 0; // Grid's left coordinate's index
    while left_idx < x_sorted.len() - 1 {
        let (left, _) = x_sorted[left_idx];

        // Find grid's right coordinate
        let (right_idx, right) = match x_sorted[left_idx + 1..]
            .iter()
            .enumerate()
            .find(|(_, &(right, _))| right > left)
        {
            Some((relative_idx, &(right, _))) => (left_idx + 1 + relative_idx, right),
            None => break,
        };

        // Iterate over all Y coordinates.
        let mut top_idx = 0; // Grid's top edge's index
        while top_idx < y_sorted.len() - 1 {
            let (top, _) = y_sorted[top_idx];
            let (bottom_idx, bottom) = match y_sorted[top_idx + 1..]
                .iter()
                .enumerate()
                .find(|(_, &(bottom, _))| bottom > top)
            {
                Some((relative_idx, &(bottom, _))) => (top_idx + 1 + relative_idx, bottom),
                None => break,
            };

            // Check if current grid is within two or more claims.
            // Note: Only need to check claims sorted before current grid's right coordinate, because for some claim
            // covers current grid it must have smaller left coordinate and all such claims already in the front.
            let mut appear_prev = false;
            for &(_, c) in &x_sorted[..right_idx] {
                if c.right >= right && c.top <= top && c.bottom >= bottom {
                    if appear_prev {
                        total_overlap += (right - left) * (bottom - top);
                        break;
                    }
                    appear_prev = true;
                }
            }

            top_idx = bottom_idx;
        }

        left_idx = right_idx;
    }

    total_overlap
}

// Find claims not overlapping with others.
fn find_non_overlap(claims: &[Claim]) -> Vec<&Claim> {
    let mut overlap: BTreeMap<&Claim, bool> = claims.iter().map(|c| (c, false)).collect();
    for i in 0..claims.len() - 1 {
        let c1 = &claims[i];

        for j in i + 1..claims.len() {
            let c2 = &claims[j];

            if intersect(c1, c2) {
                overlap.get_mut(c1).map(|v| *v = true);
                overlap.get_mut(c2).map(|v| *v = true);
            }
        }
    }

    overlap
        .iter()
        .filter_map(|(&c, &overlap)| (!overlap).then(|| c))
        .collect()
}

fn intersect(c1: &Claim, c2: &Claim) -> bool {
    !(c1.right <= c2.left || c2.right <= c1.left) && !(c1.bottom <= c2.top || c2.bottom <= c1.top)
}

fn main() -> Result<()> {
    let mut args = env::args();
    let _ = args.next().ok_or(anyhow!("arg 0 ??"));
    let input = args.next().ok_or(anyhow!("need input file"))?;

    let claims: Vec<Claim> = parse_file(input)?;

    println!("Part One: {}", fabric_overlap(&claims));
    println!("Part Two: {:?}", find_non_overlap(&claims));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Claim;

    #[test]
    fn test_parse_square() {
        let squares = [
            (
                "#1 @ 1,3: 4x4",
                Claim {
                    id: 1,
                    left: 1,
                    top: 3,
                    right: 5,
                    bottom: 7,
                },
            ),
            (
                "#2 @ 3,1: 4x4",
                Claim {
                    id: 2,
                    left: 3,
                    top: 1,
                    right: 7,
                    bottom: 5,
                },
            ),
            (
                "#3 @ 5,5: 2x2",
                Claim {
                    id: 3,
                    left: 5,
                    top: 5,
                    right: 7,
                    bottom: 7,
                },
            ),
        ];

        for (raw, expect) in squares {
            assert_eq!(Claim::from_str(raw).unwrap(), expect);
        }
    }
}
