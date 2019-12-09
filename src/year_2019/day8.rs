//! --- Day 8: Space Image Format ---

use itertools::Itertools;
use std::fmt;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

const PIXEL_BLACK: u32 = 0;
const PIXEL_WHITE: u32 = 1;
const PIXEL_TRANSPARENT: u32 = 2;

/// The Elves' spirits are lifted when they realize you have an opportunity to reboot one of their Mars rovers, and so they are curious if you would spend a brief sojourn on Mars. You land your ship near the rover.
///
/// When you reach the rover, you discover that it's already in the process of rebooting! It's just waiting for someone to enter a BIOS password. The Elf responsible for the rover takes a picture of the password (your puzzle input) and sends it to you via the Digital Sending Network.
///
/// Unfortunately, images sent via the Digital Sending Network aren't encoded with any normal encoding; instead, they're encoded in a special Space Image Format. None of the Elves seem to remember why this is the case. They send you the instructions to decode it.
///
/// Images are sent as a series of digits that each represent the color of a single pixel. The digits fill each row of the image left-to-right, then move downward to the next row, filling rows top-to-bottom until every pixel of the image is filled.
///
/// Each image actually consists of a series of identically-sized layers that are filled in this way. So, the first digit corresponds to the top-left pixel of the first layer, the second digit corresponds to the pixel to the right of that on the same layer, and so on until the last digit, which corresponds to the bottom-right pixel of the last layer.
///
/// For example, given an image 3 pixels wide and 2 pixels tall, the image data 123456789012 corresponds to the following image layers:
///
/// Layer 1: 123
///          456
///
/// Layer 2: 789
///          012
///
/// The image you received is 25 pixels wide and 6 pixels tall.
///
/// To make sure the image wasn't corrupted during transmission, the Elves would like you to find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();
    let layers = layers_from_input(input.as_str());

    let fewest_zeros_layer = layers
        .into_iter()
        .map(|layer| (layer.iter().filter(|pixel| **pixel == 0).count(), layer))
        .fold1(|fewest_zeros_layer, layer| {
            if layer.0 < fewest_zeros_layer.0 {
                layer
            } else {
                fewest_zeros_layer
            }
        })
        .expect("No layers found")
        .1;

    let number_of_ones_in_layer = fewest_zeros_layer
        .iter()
        .filter(|pixel| **pixel == 1)
        .count();

    let number_of_twos_in_layer = fewest_zeros_layer
        .iter()
        .filter(|pixel| **pixel == 2)
        .count();

    let result = number_of_ones_in_layer * number_of_twos_in_layer;

    println!(
        "On the layer that contains the fewest 0 digits, the number of 1 digits multiplied by the number of 2 digits: {}",
        result
    );
}

/// Now you're ready to decode the image. The image is rendered by stacking the layers and aligning the pixels with the same positions in each layer. The digits indicate the color of the corresponding pixel: 0 is black, 1 is white, and 2 is transparent.
///
/// The layers are rendered with the first layer in front and the last layer in back. So, if a given position has a transparent pixel in the first and second layers, a black pixel in the third layer, and a white pixel in the fourth layer, the final image would have a black pixel at that position.
///
/// For example, given an image 2 pixels wide and 2 pixels tall, the image data 0222112222120000 corresponds to the following image layers:
///
/// Layer 1: 02
///          22
///
/// Layer 2: 11
///          22
///
/// Layer 3: 22
///          12
///
/// Layer 4: 00
///          00
///
/// Then, the full image can be found by determining the top visible pixel in each position:
///
///     The top-left pixel is black because the top layer is 0.
///     The top-right pixel is white because the top layer is 2 (transparent), but the second layer is 1.
///     The bottom-left pixel is white because the top two layers are 2, but the third layer is 1.
///     The bottom-right pixel is black because the only visible pixel in that position is 0 (from layer 4).
///
/// So, the final image looks like this:
///
/// 01
/// 10
///
/// What message is produced after decoding your image?
pub fn part2() {
    let input = crate::common::read_stdin_to_string();
    let layers = layers_from_input(input.as_str());

    let image_data = ImageData(
        (0..IMAGE_HEIGHT)
            .map(|y| {
                (0..IMAGE_WIDTH)
                    .map(|x| {
                        layers
                            .iter()
                            .find(|layer| layer[x + y * IMAGE_WIDTH] != PIXEL_TRANSPARENT)
                            .map(|layer| layer[x + y * IMAGE_WIDTH])
                            .unwrap_or(PIXEL_TRANSPARENT)
                    })
                    .collect()
            })
            .collect(),
    );

    println!("The decoded image:\n{}", image_data);
}

fn layers_from_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .chars()
        .map(|character| character.to_digit(10))
        .map(|result| result.expect("Failed to parse input pixel as digit"))
        .enumerate()
        .group_by(|(index, _)| index / (IMAGE_WIDTH * IMAGE_HEIGHT))
        .into_iter()
        .map(|(_, layer)| layer.map(|(_, pixels)| pixels))
        .map(|layer| layer.collect())
        .collect()
}

struct ImageData(Vec<Vec<u32>>);

impl fmt::Display for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rendered = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| match *pixel {
                        PIXEL_BLACK => ' ',
                        PIXEL_WHITE => '█',
                        PIXEL_TRANSPARENT => '░',
                        other => panic!("Invalid pixel valid: {}", other),
                    })
                    .collect::<String>()
            })
            .join("\n");

        write!(f, "{}", rendered)
    }
}
