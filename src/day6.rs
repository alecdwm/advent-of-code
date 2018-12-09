//! The device on your wrist beeps several times, and once again you feel like you're falling.

use std::cmp;
use std::collections::BTreeMap;
use std::i64;

/// "Situation critical," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."
///
/// The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.
///
/// If they're dangerous, maybe you can minimize the danger by finding the coordinate that gives the largest distance from the other points.
///
/// Using only the Manhattan distance, determine the area around each coordinate by counting the number of integer X,Y locations that are closest to that coordinate (and aren't tied in distance to any other coordinate).
///
/// Your goal is to find the size of the largest area that isn't infinite. For example, consider the following list of coordinates:
///
/// 1, 1
/// 1, 6
/// 8, 3
/// 3, 4
/// 5, 5
/// 8, 9
///
/// If we name these coordinates A through F, we can draw them on a grid, putting 0,0 at the top left:
///
/// ..........
/// .A........
/// ..........
/// ........C.
/// ...D......
/// .....E....
/// .B........
/// ..........
/// ..........
/// ........F.
///
/// This view is partial - the actual grid extends infinitely in all directions. Using the Manhattan distance, each location's closest coordinate can be determined, shown here in lowercase:
///
/// aaaaa.cccc
/// aAaaa.cccc
/// aaaddecccc
/// aadddeccCc
/// ..dDdeeccc
/// bb.deEeecc
/// bBb.eeee..
/// bbb.eeefff
/// bbb.eeffff
/// bbb.ffffFf
///
/// Locations shown as . are equally far from two or more coordinates, and so they don't count as being closest to any.
///
/// In this example, the areas of coordinates A, B, C, and F are infinite - while not shown here, their areas extend forever outside the visible grid. However, the areas of coordinates D and E are finite: D is closest to 9 locations, and E is closest to 17 (both including the coordinate's location itself). Therefore, in this example, the size of the largest area is 17.
///
/// What is the size of the largest area that isn't infinite?
pub fn part1() {
    let input = ::common::read_stdin_to_string();

    let coords = input_to_coords(input);
    let bounds = get_bounds(&coords);
    let areas = calculate_areas(&coords, bounds);

    let largest_area = areas.values().fold(0, |acc, v| cmp::max(acc, *v));

    println!(
        "the size of the largest area that isn't infinite: {}",
        largest_area
    );
}

fn input_to_coords(input: String) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(',').collect();
            (
                split[0].trim().parse().unwrap(),
                split[1].trim().parse().unwrap(),
            )
        }).collect()
}

fn get_bounds(coords: &[(i64, i64)]) -> ((i64, i64), (i64, i64)) {
    coords.iter().fold(
        ((i64::MAX, i64::MAX), (0, 0)),
        |(min_coord, max_coord), (x, y)| {
            (
                (cmp::min(min_coord.0, *x), cmp::min(min_coord.1, *y)),
                (cmp::max(max_coord.0, *x), cmp::max(max_coord.1, *y)),
            )
        },
    )
}

fn calculate_areas(
    coords: &[(i64, i64)],
    (min_coord, max_coord): ((i64, i64), (i64, i64)),
) -> BTreeMap<(i64, i64), i64> {
    let mut areas = BTreeMap::new();

    for x in min_coord.0..=max_coord.0 {
        for y in min_coord.1..=max_coord.1 {
            let (_, coord) =
                coords
                    .iter()
                    .fold((i64::MAX, None), |(closest, closest_coord), coord| {
                        let distance = taxicab_distance((x, y), *coord);
                        match distance {
                            d if d < closest => (d, Some(*coord)),
                            d if d == closest => (d, None),
                            _ => (closest, closest_coord),
                        }
                    });

            if let Some(coord) = coord {
                if x == min_coord.0 || x == max_coord.0 {
                    *areas.entry(coord).or_insert(0) = -1;
                    continue;
                }
                if y == min_coord.1 || y == max_coord.1 {
                    *areas.entry(coord).or_insert(0) = -1;
                    continue;
                }
                let entry = areas.entry(coord).or_insert(0);
                if *entry == -1 {
                    continue;
                }
                *entry += 1
            }
        }
    }

    areas
}

fn taxicab_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (cmp::max(a.0, b.0) - cmp::min(a.0, b.0)) + (cmp::max(a.1, b.1) - cmp::min(a.1, b.1))
}
