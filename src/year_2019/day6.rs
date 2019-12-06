//! --- Day 6: Universal Orbit Map ---

use std::collections::HashMap;

/// You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).
///
/// Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:
///
///                   \
///                    \
///                     |
///                     |
/// AAA--> o            o <--BBB
///                     |
///                     |
///                    /
///                   /
///
/// In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".
///
/// Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.
///
/// Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.
///
/// For example, suppose you have the following map:
///
/// COM)B
/// B)C
/// C)D
/// D)E
/// E)F
/// B)G
/// G)H
/// D)I
/// E)J
/// J)K
/// K)L
///
/// Visually, the above map of orbits looks like this:
///
///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I
///
/// In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.
///
/// Here, we can count the total number of orbits as follows:
///
///     D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
///     L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
///     COM orbits nothing.
///
/// The total number of direct and indirect orbits in this example is 42.
///
/// What is the total number of direct and indirect orbits in your map data?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();
    let orbit_map = OrbitMap::from(input.as_str());
    let total_orbits = orbit_map.orbit_count_checksum();

    println!(
        "The total number of direct and indirect orbits in your map data: {}",
        total_orbits
    );
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct OrbitMapID(String);

#[derive(Debug, Default)]
struct OrbitMap {
    bodies: HashMap<OrbitMapID, OrbitMapBody>,
}

impl OrbitMap {
    fn add_orbit_relation(&mut self, target: OrbitMapID, source: OrbitMapID) {
        // insert target if it doesn't exist
        self.bodies.entry(target.clone()).or_default();

        // insert source if it doesn't exist
        let source = self.bodies.entry(source).or_default();

        // set source.parent to target
        if let Some(parent) = &source.parent {
            panic!(
                "Failed adding orbit relation, body already has a parent: {:?}",
                *parent
            );
        }
        source.parent = Some(target);
    }

    fn orbit_count_checksum(&self) -> u32 {
        self.bodies
            .values()
            .map(|body| body.count_parents(self))
            .sum()
    }
}

impl From<&str> for OrbitMap {
    fn from(string: &str) -> Self {
        let mut orbit_map: Self = Default::default();

        string.trim().split('\n').for_each(|orbit| {
            let mut bodies = orbit.split(')').map(|id| OrbitMapID(id.to_string()));
            orbit_map.add_orbit_relation(
                bodies
                    .next()
                    .unwrap_or_else(|| panic!("Failed to parse orbit relation {}", orbit)),
                bodies
                    .next()
                    .unwrap_or_else(|| panic!("Failed to parse orbit relation {}", orbit)),
            );
        });

        orbit_map
    }
}

#[derive(Debug, Default)]
struct OrbitMapBody {
    parent: Option<OrbitMapID>,
}

impl OrbitMapBody {
    fn count_parents(&self, map: &OrbitMap) -> u32 {
        let mut count = 0;
        let mut body = self;
        while let Some(parent) = &body.parent {
            count += 1;
            body = match map.bodies.get(parent) {
                Some(parent) => parent,
                None => panic!("OrbitMapBody with id {:?} not found", parent),
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_count_checksum() {
        let example = (
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L",
            42,
        );

        assert_eq!(OrbitMap::from(example.0).orbit_count_checksum(), example.1);
    }
}
