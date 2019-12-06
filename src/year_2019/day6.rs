//! --- Day 6: Universal Orbit Map ---

use std::collections::HashMap;
use std::fmt;

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

/// Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).
///
/// You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An orbital transfer lets you move from any object to an object orbiting or orbited by that object.
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
/// K)YOU
/// I)SAN
///
/// Visually, the above map of orbits looks like this:
///
///                           YOU
///                          /
///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I - SAN
///
/// In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:
///
///     K to J
///     J to E
///     E to D
///     D to I
///
/// Afterward, the map of orbits looks like this:
///
///         G - H       J - K - L
///        /           /
/// COM - B - C - D - E - F
///                \
///                 I - SAN
///                  \
///                   YOU
///
/// What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.libunwind
pub fn part2() {
    let input = crate::common::read_stdin_to_string();
    let orbit_map = OrbitMap::from(input.as_str());

    let minimum_transfers =
        orbit_map.minimum_transfers(&OrbitMapID::from("YOU"), &OrbitMapID::from("SAN"));

    println!(
        "The minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting: {}",
        minimum_transfers
    );
}

#[derive(Debug, Default)]
struct OrbitMap {
    bodies: HashMap<OrbitMapID, OrbitMapBody>,
}

impl OrbitMap {
    fn get_body(&self, id: &OrbitMapID) -> &OrbitMapBody {
        self.bodies
            .get(id)
            .unwrap_or_else(|| panic!("{} body not found in OrbitMap", id))
    }

    fn get_body_parent_id(&self, id: &OrbitMapID) -> &OrbitMapID {
        self.get_body(id)
            .parent
            .as_ref()
            .unwrap_or_else(|| panic!("{} body missing parent", id))
    }

    fn add_orbit_relation(&mut self, target: &OrbitMapID, source: &OrbitMapID) {
        // insert target if it doesn't exist
        self.bodies
            .entry(target.clone())
            .or_insert_with(|| OrbitMapBody::new(target));

        // insert source if it doesn't exist
        let source = self
            .bodies
            .entry(source.clone())
            .or_insert_with(|| OrbitMapBody::new(source));

        // set source.parent to target
        if let Some(parent) = &source.parent {
            panic!(
                "Failed adding orbit relation, body already has a parent: {}",
                parent
            );
        }
        source.parent = Some(target.clone());
    }

    fn orbit_count_checksum(&self) -> usize {
        self.bodies
            .values()
            .map(|body| body.parents(self).count())
            .sum()
    }

    fn minimum_transfers(&self, source_id: &OrbitMapID, target_id: &OrbitMapID) -> usize {
        let source = self.get_body(self.get_body_parent_id(source_id));
        let target = self.get_body(self.get_body_parent_id(target_id));

        let source_parents = source.parents(&self).collect::<Vec<_>>();
        let target_parents = target.parents(&self).collect::<Vec<_>>();

        let mut minimum_transfers = 0;
        let mut common_parent: Option<OrbitMapID> = None;

        for parent in source_parents.iter() {
            minimum_transfers += 1;

            if target_parents
                .iter()
                .any(|target_parent| parent.id == target_parent.id)
            {
                common_parent = Some(parent.id.clone());
                break;
            }
        }

        let common_parent_id = common_parent.unwrap_or_else(|| {
            panic!(
                "Failed to find common parent of {} and {}",
                source_id, target_id
            )
        });

        for parent in target_parents.iter() {
            minimum_transfers += 1;

            if parent.id == common_parent_id {
                break;
            }
        }

        minimum_transfers
    }
}

impl From<&str> for OrbitMap {
    fn from(string: &str) -> Self {
        let mut orbit_map: Self = Default::default();

        string.trim().split('\n').for_each(|orbit| {
            let mut bodies = orbit.split(')').map(|id| OrbitMapID(id.to_string()));
            orbit_map.add_orbit_relation(
                &bodies
                    .next()
                    .unwrap_or_else(|| panic!("Failed to parse orbit relation {}", orbit)),
                &bodies
                    .next()
                    .unwrap_or_else(|| panic!("Failed to parse orbit relation {}", orbit)),
            );
        });

        orbit_map
    }
}

#[derive(Debug)]
struct OrbitMapBody {
    id: OrbitMapID,
    parent: Option<OrbitMapID>,
}

impl OrbitMapBody {
    fn new(id: &OrbitMapID) -> Self {
        Self {
            id: id.clone(),
            parent: None,
        }
    }

    fn parents<'a>(&self, map: &'a OrbitMap) -> OrbitMapBodyParentIterator<'a> {
        OrbitMapBodyParentIterator {
            map,
            next_parent_id: self.parent.clone(),
        }
    }
}

struct OrbitMapBodyParentIterator<'a> {
    map: &'a OrbitMap,
    next_parent_id: Option<OrbitMapID>,
}

impl<'a> Iterator for OrbitMapBodyParentIterator<'a> {
    type Item = &'a OrbitMapBody;

    fn next(&mut self) -> Option<Self::Item> {
        let parent = match self.next_parent_id.as_ref() {
            Some(next_parent_id) => match self.map.bodies.get(&next_parent_id) {
                Some(parent) => parent,
                None => return None,
            },
            None => return None,
        };

        self.next_parent_id = parent.parent.clone();

        Some(parent)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct OrbitMapID(String);

impl From<&str> for OrbitMapID {
    fn from(string: &str) -> Self {
        Self(string.to_string())
    }
}

impl fmt::Display for OrbitMapID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
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

    #[test]
    fn test_orbital_transfers() {
        let example = (
            "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN",
            4,
        );

        assert_eq!(
            OrbitMap::from(example.0)
                .minimum_transfers(&OrbitMapID::from("YOU"), &OrbitMapID::from("SAN")),
            example.1
        );
    }
}
