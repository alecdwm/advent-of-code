//! --- Day 10: Monitoring Station ---

/// You fly into the asteroid belt and reach the Ceres monitoring station. The Elves here have an emergency: they're having trouble tracking all of the asteroids and can't be sure they're safe.
///
/// The Elves would like to build a new monitoring station in a nearby area of space; they hand you a map of all of the asteroids in that region (your puzzle input).
///
/// The map indicates whether each position is empty (.) or contains an asteroid (#). The asteroids are much smaller than they appear on the map, and every asteroid is exactly in the center of its marked position. The asteroids can be described with X,Y coordinates where X is the distance from the left edge and Y is the distance from the top edge (so the top-left corner is 0,0 and the position immediately to its right is 1,0).
///
/// Your job is to figure out which asteroid would be the best place to build a new monitoring station. A monitoring station can detect any asteroid to which it has direct line of sight - that is, there cannot be another asteroid exactly between them. This line of sight can be at any angle, not just lines aligned to the grid or diagonally. The best location is the asteroid that can detect the largest number of other asteroids.
///
/// For example, consider the following map:
///
/// .#..#
/// .....
/// #####
/// ....#
/// ...##
///
/// The best location for a new monitoring station on this map is the highlighted asteroid at 3,4 because it can detect 8 asteroids, more than any other location. (The only asteroid it cannot detect is the one at 1,0; its view of this asteroid is blocked by the asteroid at 2,2.) All other asteroids are worse locations; they can detect 7 or fewer other asteroids. Here is the number of other asteroids a monitoring station on each asteroid could detect:
///
/// .7..7
/// .....
/// 67775
/// ....7
/// ...87
///
/// Here is an asteroid (#) and some examples of the ways its line of sight might be blocked. If there were another asteroid at the location of a capital letter, the locations marked with the corresponding lowercase letter would be blocked and could not be detected:
///
/// #.........
/// ...A......
/// ...B..a...
/// .EDCG....a
/// ..F.c.b...
/// .....c....
/// ..efd.c.gb
/// .......c..
/// ....f...c.
/// ...e..d..c
///
/// Here are some larger examples:
///
///     Best is 5,8 with 33 other asteroids detected:
///
///     ......#.#.
///     #..#.#....
///     ..#######.
///     .#.#.###..
///     .#..#.....
///     ..#....#.#
///     #..#....#.
///     .##.#..###
///     ##...#..#.
///     .#....####
///
///     Best is 1,2 with 35 other asteroids detected:
///
///     #.#...#.#.
///     .###....#.
///     .#....#...
///     ##.#.#.#.#
///     ....#.#.#.
///     .##..###.#
///     ..#...##..
///     ..##....##
///     ......#...
///     .####.###.
///
///     Best is 6,3 with 41 other asteroids detected:
///
///     .#..#..###
///     ####.###.#
///     ....###.#.
///     ..###.##.#
///     ##.##.#.#.
///     ....###..#
///     ..#.#..#.#
///     #..#.#.###
///     .##...##.#
///     .....#.#..
///
///     Best is 11,13 with 210 other asteroids detected:
///
///     .#..##.###...#######
///     ##.############..##.
///     .#.######.########.#
///     .###.#######.####.#.
///     #####.##.#.##.###.##
///     ..#####..#.#########
///     ####################
///     #.####....###.#.#.##
///     ##.#################
///     #####.##.###..####..
///     ..######..##.#######
///     ####.##.####...##..#
///     .#####..#.######.###
///     ##...#.##########...
///     #.##########.#######
///     .####.#.###.###.#.##
///     ....##.##.###..#####
///     .#.#.###########.###
///     #.#.#.#####.####.###
///     ###.##.####.##.#..##
///
/// Find the best location for a new monitoring station. How many other asteroids can be detected from that location?

pub fn part1() {
    let input = crate::common::read_stdin_to_string();

    let map = AsteroidMap::from(input.as_str());
    let (number_of_asteroids, _) = map.calculate_best_monitoring_station();

    println!(
        "The number of other asteroids that can be detected from the best location for a new monitoring station: {}",
        number_of_asteroids
    );
}

#[derive(Debug)]
struct AsteroidMap(Vec<Point>);

impl AsteroidMap {
    fn calculate_best_monitoring_station(&self) -> (usize, Point) {
        self.0
            .iter()
            .copied()
            .map(|asteroid| (self.number_of_visible_asteroids(asteroid), asteroid))
            .max_by(|(visible_asteroids_one, _), (visible_asteroids_two, _)| {
                visible_asteroids_one.cmp(visible_asteroids_two)
            })
            .expect("Failed to find best monitoring station")
    }

    fn number_of_visible_asteroids(&self, from: Point) -> usize {
        let mut vectors: Vec<_> = self
            .0
            .iter()
            .copied()
            .filter(|asteroid| *asteroid != from)
            .map(|asteroid| Vector::from_points(from, asteroid))
            .map(|vector| vector.angle())
            .collect();

        vectors.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        vectors.dedup();
        vectors.len()
    }
}

impl From<&str> for AsteroidMap {
    fn from(string: &str) -> Self {
        Self(
            string
                .trim()
                .split('\n')
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, character)| *character == '#')
                        .map(move |(x, _)| Point {
                            x: x as i64,
                            y: y as i64,
                        })
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn from_points(one: Point, two: Point) -> Self {
        Self {
            x: two.x - one.x,
            y: two.y - one.y,
        }
    }

    fn angle(&self) -> Angle {
        Angle {
            theta: (self.y as f64 / self.x as f64).atan(),
            side: if self.x >= 0 { Side::Right } else { Side::Left },
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
struct Angle {
    theta: f64,

    // -0.0_f64 == 0.0_f64, but in this context we want to distinguish between them
    side: Side,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
enum Side {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        let examples = [
            (
                "
.#..#
.....
#####
....#
...##
",
                Point { x: 3, y: 4 },
                8,
            ),
            (
                "
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
            ",
                Point { x: 5, y: 8 },
                33,
            ),
            (
                "
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
            ",
                Point { x: 1, y: 2 },
                35,
            ),
            (
                "
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
            ",
                Point { x: 6, y: 3 },
                41,
            ),
            (
                "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
            ",
                Point { x: 11, y: 13 },
                210,
            ),
        ];

        for example in &examples {
            let best_station = AsteroidMap::from(example.0).calculate_best_monitoring_station();
            assert_eq!(best_station.1, example.1);
            assert_eq!(best_station.0, example.2);
        }
    }
}
