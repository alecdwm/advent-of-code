//! --- Day 3: Crossed Wires ---

/// The gravity assist was successful, and you're well on your way to the Venus refuelling station. During the rush back on Earth, the fuel management system wasn't completely installed, so that's next on the priority list.
///
/// Opening the front panel reveals a jumble of wires. Specifically, two wires are connected to a central port and extend outward on a grid. You trace the path each wire takes as it leaves the central port, one wire per line of text (your puzzle input).
///
/// The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need to find the intersection point closest to the central port. Because the wires are on a grid, use the Manhattan distance for this measurement. While the wires do technically cross right at the central port where they both start, this point does not count, nor does a wire count as crossing with itself.
///
/// For example, if the first wire's path is R8,U5,L5,D3, then starting from the central port (o), it goes right 8, up 5, left 5, and finally down 3:
///
/// ...........
/// ...........
/// ...........
/// ....+----+.
/// ....|....|.
/// ....|....|.
/// ....|....|.
/// .........|.
/// .o-------+.
/// ...........
///
/// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down 4, and left 4:
///
/// ...........
/// .+-----+...
/// .|.....|...
/// .|..+--X-+.
/// .|..|..|.|.
/// .|.-X--+.|.
/// .|..|....|.
/// .|.......|.
/// .o-------+.
/// ...........
///
/// These wires cross at two locations (marked X), but the lower-left one is closer to the central port: its distance is 3 + 3 = 6.
///
/// Here are a few more examples:
///
///     R75,D30,R83,U83,L12,D49,R71,U7,L72
///     U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
///     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
///     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
///
/// What is the Manhattan distance from the central port to the closest intersection?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();
    let wires = Wire::parse_wires(input.as_str());

    let first_wire = wires.get(0).expect("Missing first wire");
    let second_wire = wires.get(1).expect("Missing second wire");

    let origin = Point::zero();
    let min_distance = first_wire
        .intersections(second_wire)
        .iter()
        .map(|intersection| origin.manhattan_distance(intersection))
        .min()
        .expect("No intersections found!");

    println!(
        "The Manhattan distance from the central port to the closest intersection: {}",
        min_distance
    );
}

#[derive(Debug)]
struct Wire {
    points: Vec<Point>,
}

impl Default for Wire {
    fn default() -> Self {
        Self {
            points: vec![Point::zero()],
        }
    }
}

impl Wire {
    fn parse_wires(serialized: &str) -> Vec<Self> {
        serialized.trim().split("\n").map(Wire::from).collect()
    }

    fn intersections(&self, target: &Wire) -> Vec<Point> {
        let mut intersections = Vec::new();

        for line_segment in self.points.windows(2) {
            for target_line_segment in target.points.windows(2) {
                let start = line_segment[0];
                let end = line_segment[1];
                let target_start = target_line_segment[0];
                let target_end = target_line_segment[1];

                if start.x == end.x && target_start.x == target_end.x {
                    continue;
                }
                if start.y == end.y && target_start.y == target_end.y {
                    continue;
                }

                let min_x = i64::min(start.x, end.x);
                let max_x = i64::max(start.x, end.x);
                let min_y = i64::min(start.y, end.y);
                let max_y = i64::max(start.y, end.y);

                let target_min_x = i64::min(target_start.x, target_end.x);
                let target_max_x = i64::max(target_start.x, target_end.x);
                let target_min_y = i64::min(target_start.y, target_end.y);
                let target_max_y = i64::max(target_start.y, target_end.y);

                if min_x == max_x {
                    assert_eq!(target_min_y, target_max_y);
                    let x = min_x;
                    let target_y = target_min_y;

                    if target_min_x <= x
                        && x <= target_max_x
                        && min_y <= target_y
                        && target_y <= max_y
                    {
                        let intersection = Point::new(x, target_y);
                        if intersection == Point::zero() {
                            continue;
                        }
                        intersections.push(intersection);
                    }
                } else {
                    assert_eq!(min_y, max_y);
                    assert_eq!(target_min_x, target_max_x);
                    let y = min_y;
                    let target_x = target_min_x;

                    if target_min_y <= y
                        && y <= target_max_y
                        && min_x <= target_x
                        && target_x <= max_x
                    {
                        let intersection = Point::new(target_x, y);
                        if intersection == Point::zero() {
                            continue;
                        }
                        intersections.push(intersection);
                    }
                }
            }
        }

        intersections
    }

    fn add_point_from_segment(&mut self, wire_segment: &str) {
        let last_point = self.points.last().cloned().unwrap_or_else(Point::zero);

        let mut chars = wire_segment.chars();
        let direction: WireSegmentDirection = chars
            .next()
            .unwrap_or_else(|| panic!("Failed to read wire segment {}", wire_segment))
            .into();
        let distance: i64 = chars
            .as_str()
            .parse()
            .expect("Failed to parse distance as u64");

        match direction {
            WireSegmentDirection::Up => self
                .points
                .push(Point::new(last_point.x, last_point.y + distance)),
            WireSegmentDirection::Down => self
                .points
                .push(Point::new(last_point.x, last_point.y - distance)),
            WireSegmentDirection::Left => self
                .points
                .push(Point::new(last_point.x - distance, last_point.y)),
            WireSegmentDirection::Right => self
                .points
                .push(Point::new(last_point.x + distance, last_point.y)),
        }
    }
}

impl From<&str> for Wire {
    fn from(wire_serialized: &str) -> Self {
        let mut wire: Self = Default::default();

        wire_serialized
            .trim()
            .split(",")
            .for_each(|wire_segment| wire.add_point_from_segment(wire_segment));

        wire
    }
}

enum WireSegmentDirection {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for WireSegmentDirection {
    fn from(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            other => panic!(
                "Invalid wire segment direction (must be U,D,L or R): {}",
                other
            ),
        }
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Default::default()
    }

    fn manhattan_distance(&self, to: &Point) -> i64 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let examples = [
            (Point::new(-5, -5), Point::new(5, 5), 20),
            (Point::new(0, 0), Point::new(10, 0), 10),
            (Point::new(0, 0), Point::new(0, 10), 10),
            (Point::new(0, 0), Point::new(10, 10), 20),
            (Point::new(0, 0), Point::new(-10, 0), 10),
            (Point::new(0, 0), Point::new(0, -10), 10),
            (Point::new(0, 0), Point::new(-10, -10), 20),
        ];

        for example in &examples {
            assert_eq!(example.0.manhattan_distance(&example.1), example.2);
        }
    }

    #[test]
    fn test_part1_examples() {
        let examples = [
            ("R8,U5,L5,D3\nU7,R6,D4,L4", 6),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                159,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ];

        for example in &examples {
            let wires = Wire::parse_wires(example.0);

            let first_wire = wires.get(0).expect("Missing first wire");
            let second_wire = wires.get(1).expect("Missing second wire");

            let origin = Point::zero();
            let min_distance = first_wire
                .intersections(second_wire)
                .iter()
                .map(|intersection| origin.manhattan_distance(intersection))
                .min()
                .expect("No intersections found!");

            assert_eq!(min_distance, example.1);
        }
    }
}
