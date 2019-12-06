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
    let minimum_transfers = orbit_map.minimum_transfers("SAN", "YOU");

    println!(
        "The minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting: {}",
        minimum_transfers
    );
}

#[derive(Debug, Default)]
struct OrbitMap<'a> {
    bodies: HashMap<&'a str, OrbitMapBody<'a>>,
}

impl<'a> OrbitMap<'a> {
    fn get_body(&self, id: &str) -> &OrbitMapBody {
        self.bodies
            .get(id)
            .unwrap_or_else(|| panic!("{} body not found in OrbitMap", id))
    }

    fn get_body_parent(&self, id: &str) -> &OrbitMapBody {
        let parent_id = self
            .get_body(id)
            .parent
            .unwrap_or_else(|| panic!("{} body missing parent", id));

        self.get_body(parent_id)
    }

    fn add_orbit_relation(&mut self, target_id: &'a str, source_id: &'a str) {
        // insert target if it doesn't exist
        self.bodies
            .entry(target_id)
            .or_insert_with(|| OrbitMapBody::new(target_id));

        // insert source if it doesn't exist
        let source = self
            .bodies
            .entry(source_id)
            .or_insert_with(|| OrbitMapBody::new(source_id));

        // set source.parent to target
        if let Some(parent_id) = &source.parent {
            panic!(
                "Failed adding orbit relation, source body {} already has a parent: {}",
                source_id, parent_id
            );
        }
        source.parent = Some(target_id);
    }

    fn orbit_count_checksum(&self) -> usize {
        self.bodies
            .values()
            .map(|body| body.parents(self).count())
            .sum()
    }

    fn minimum_transfers(&self, target_id: &'a str, source_id: &'a str) -> usize {
        let source = self.get_body_parent(source_id);
        let target = self.get_body_parent(target_id);

        let source_parents: Vec<_> = source.parents(self).collect();
        let target_parents: Vec<_> = target.parents(self).collect();

        let mut minimum_transfers = 0;
        let mut common_parent = None;

        for parent in source_parents.iter() {
            minimum_transfers += 1;

            if target_parents
                .iter()
                .any(|target_parent| parent.id == target_parent.id)
            {
                common_parent = Some(parent.id);
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

impl<'a> From<&'a str> for OrbitMap<'a> {
    fn from(string: &'a str) -> Self {
        string
            .trim()
            .split('\n')
            .map(|orbit| (orbit, orbit.split(')')))
            .fold(Default::default(), |mut orbit_map, (orbit, mut bodies)| {
                orbit_map.add_orbit_relation(
                    bodies
                        .next()
                        .unwrap_or_else(|| panic!("Failed to parse orbit relation {}", orbit)),
                    bodies
                        .next()
                        .unwrap_or_else(|| panic!("Failed to parse orbit relation {}", orbit)),
                );

                orbit_map
            })
    }
}

#[derive(Debug)]
struct OrbitMapBody<'a> {
    id: &'a str,
    parent: Option<&'a str>,
}

impl<'a> OrbitMapBody<'a> {
    fn new(id: &'a str) -> Self {
        Self { id, parent: None }
    }

    fn parents(&self, map: &'a OrbitMap) -> OrbitMapBodyParentIterator<'a> {
        OrbitMapBodyParentIterator {
            map,
            next_parent_id: self.parent,
        }
    }
}

struct OrbitMapBodyParentIterator<'a> {
    map: &'a OrbitMap<'a>,
    next_parent_id: Option<&'a str>,
}

impl<'a> Iterator for OrbitMapBodyParentIterator<'a> {
    type Item = &'a OrbitMapBody<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_parent = match self.next_parent_id {
            Some(next_parent_id) => match self.map.bodies.get(next_parent_id) {
                Some(next_parent) => next_parent,
                None => return None,
            },
            None => return None,
        };

        self.next_parent_id = next_parent.parent;

        Some(next_parent)
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
            OrbitMap::from(example.0).minimum_transfers("SAN", "YOU"),
            example.1
        );
    }
}
