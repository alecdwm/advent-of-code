//! The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to someone who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night). Unfortunately, anomalies are still affecting them - nobody can even agree on how to cut the fabric.
//!
//! The whole piece of fabric they're working on is a very large square - at least 1000 inches on each side.

use std::collections::BTreeMap;
use std::num::ParseIntError;
use std::str::FromStr;

/// Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:
///
///     The number of inches between the left edge of the fabric and the left edge of the rectangle.
///     The number of inches between the top edge of the fabric and the top edge of the rectangle.
///     The width of the rectangle in inches.
///     The height of the rectangle in inches.
///
/// A claim like #123 @ 3,2: 5x4 means that claim ID 123 specifies a rectangle 3 inches from the left edge, 2 inches from the top edge, 5 inches wide, and 4 inches tall. Visually, it claims the square inches of fabric represented by # (and ignores the square inches of fabric represented by .) in the diagram below:
///
/// ...........
/// ...........
/// ...#####...
/// ...#####...
/// ...#####...
/// ...#####...
/// ...........
/// ...........
/// ...........
///
/// The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:
///
/// #1 @ 1,3: 4x4
/// #2 @ 3,1: 4x4
/// #3 @ 5,5: 2x2
///
/// Visually, these claim the following areas:
///
/// ........
/// ...2222.
/// ...2222.
/// .11XX22.
/// .11XX22.
/// .111133.
/// .111133.
/// ........
///
/// The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)
///
/// If the Elves all proceed with their own plans, none of them will have enough fabric. How many square inches of fabric are within two or more claims?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();

    let mut fabric: BTreeMap<(i64, i64), u8> = BTreeMap::new();

    for line in input.lines() {
        let claim: FabricClaim = line.parse().expect("Parsing fabric claim");
        for w in 0..claim.width {
            for h in 0..claim.height {
                let index = (claim.pos_x + w, claim.pos_y + h);
                *fabric.entry(index).or_insert(0) += 1;
            }
        }
    }

    let contested_square_inches = fabric.values().filter(|v| **v > 1).count();

    println!(
        "the number of square inches of fabric within two or more claims: {}",
        contested_square_inches
    );
}

/// Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be able to make Santa's suit after all!
///
/// For example, in the claims above, only claim 3 is intact after all claims are made.
///
/// What is the ID of the only claim that doesn't overlap?
pub fn part2() {
    let input = crate::common::read_stdin_to_string();

    let mut fabric: BTreeMap<(i64, i64), u8> = BTreeMap::new();
    let mut claims: Vec<FabricClaim> = Vec::new();

    for line in input.lines() {
        let claim: FabricClaim = line.parse().expect("Parsing fabric claim");
        for w in 0..claim.width {
            for h in 0..claim.height {
                let index = (claim.pos_x + w, claim.pos_y + h);
                *fabric.entry(index).or_insert(0) += 1;
            }
        }
        claims.push(line.parse().expect("Parsing fabric claim"));
    }

    let mut free_claim_id = -1;

    'claim_loop: for claim in claims.iter() {
        for w in 0..claim.width {
            for h in 0..claim.height {
                let index = (claim.pos_x + w, claim.pos_y + h);
                if fabric[&index] > 1 {
                    continue 'claim_loop;
                }
            }
        }
        free_claim_id = claim.id;
    }

    println!(
        "the ID of the only claim that doesn't overlap: {}",
        free_claim_id
    );
}

#[derive(Debug)]
struct FabricClaim {
    id: i64,
    pos_x: i64,
    pos_y: i64,
    width: i64,
    height: i64,
}

impl FromStr for FabricClaim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.trim_start_matches('#').split('@').collect();
        let id = split[0].trim().parse::<i64>()?;

        let split: Vec<&str> = split[1].split(':').collect();

        let pos: Vec<&str> = split[0].split(',').collect();
        let pos_x = pos[0].trim().parse::<i64>()?;
        let pos_y = pos[1].trim().parse::<i64>()?;

        let size: Vec<&str> = split[1].split('x').collect();
        let width = size[0].trim().parse::<i64>()?;
        let height = size[1].trim().parse::<i64>()?;

        Ok(FabricClaim {
            id,
            pos_x,
            pos_y,
            width,
            height,
        })
    }
}
