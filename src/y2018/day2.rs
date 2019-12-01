//! You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
//!
//! Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that so many people have chimneys, maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of suit that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"
//!
//! "Wouldn't they have had enough fabric to fill several boxes in the warehouse? They'd be stored together, so the box IDs should be similar. Too bad it would take forever to search the warehouse for two similar box IDs..." They walk too far away to hear any more.

use std::collections::BTreeMap;

/// Late at night, you sneak to the warehouse - who knows what kinds of paradoxes you could cause if you were discovered - and use your fancy wrist device to quickly scan every box and produce a list of the likely candidates (your puzzle input).
///
/// To make sure you didn't miss any, you scan the likely candidate boxes again, counting the number that have an ID containing exactly two of any letter and then separately counting those with exactly three of any letter. You can multiply those two counts together to get a rudimentary checksum and compare it to what your device predicts.
///
/// For example, if you see the following box IDs:
///
///     abcdef contains no letters that appear exactly two or three times.
///     bababc contains two a and three b, so it counts for both.
///     abbcde contains two b, but no letter appears exactly three times.
///     abcccd contains three c, but no letter appears exactly two times.
///     aabcdd contains two a and two d, but it only counts once.
///     abcdee contains two e.
///     ababab contains three a and three b, but it only counts once.
///
/// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.
///
/// What is the checksum for your list of box IDs?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();

    let mut two_letter_checksum_component: i64 = 0;
    let mut three_letter_checksum_component: i64 = 0;

    let mut seen_letter_counts: BTreeMap<char, i64> = BTreeMap::new();

    for box_id in input.lines() {
        for letter in box_id.chars() {
            let count = (match seen_letter_counts.get(&letter) {
                Some(count) => count,
                None => &0,
            }) + 1;
            seen_letter_counts.insert(letter, count);
        }

        let mut seen_two = false;
        let mut seen_three = false;
        for count in seen_letter_counts.values() {
            if !seen_two && *count == 2 {
                seen_two = true;
                two_letter_checksum_component += 1;
            }
            if !seen_three && *count == 3 {
                seen_three = true;
                three_letter_checksum_component += 1;
            }
            if seen_two && seen_three {
                break;
            }
        }

        seen_letter_counts.clear();
    }

    let checksum = two_letter_checksum_component * three_letter_checksum_component;

    println!("the checksum for your list of box IDs: {}", checksum);
}

/// Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.
///
/// The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:
///
/// abcde
/// fghij
/// klmno
/// pqrst
/// fguij
/// axcye
/// wvxyz
///
/// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.
///
/// What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)
pub fn part2() {
    let input = crate::common::read_stdin_to_string();

    let matches = find_part2_matches(&input).expect("No matches found");

    let common_letters: String = matches
        .0
        .chars()
        .zip(matches.1.chars())
        .filter(|(letter_1, letter_2)| letter_1 == letter_2)
        .map(|letters| letters.0)
        .collect();

    println!(
        "the common letters between the two correct box IDs: {}",
        common_letters
    );
}

fn find_part2_matches(input: &str) -> Option<(String, String)> {
    for box_id_1 in input.lines() {
        'test_inner: for box_id_2 in input.lines() {
            if box_id_1 == box_id_2 {
                continue;
            }

            let mut differences = 0;
            for (letter_1, letter_2) in box_id_1.chars().zip(box_id_2.chars()) {
                if letter_1 != letter_2 {
                    differences += 1;
                    if differences > 1 {
                        continue 'test_inner;
                    }
                }
            }
            if differences == 1 {
                return Some((box_id_1.to_string(), box_id_2.to_string()));
            }
        }
    }
    None
}
