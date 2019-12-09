//! --- Day 8: Space Image Format ---

use itertools::Itertools;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

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

    let layers: Vec<String> = input
        .trim()
        .chars()
        .enumerate()
        .group_by(|(index, _)| index / (IMAGE_WIDTH * IMAGE_HEIGHT))
        .into_iter()
        .map(|(_, group)| group)
        .map(|group| group.map(|(_, characters)| characters))
        .map(|group| group.collect())
        .collect();

    let fewest_zeros_layer = layers
        .into_iter()
        .map(|layer| (layer.chars().filter(|c| *c == '0').count(), layer))
        .fold1(|fewest_zeros_layer, layer| {
            if layer.0 < fewest_zeros_layer.0 {
                layer
            } else {
                fewest_zeros_layer
            }
        })
        .expect("No layers found")
        .1;

    let layer_ones = fewest_zeros_layer.chars().filter(|c| *c == '1').count();
    let layer_twos = fewest_zeros_layer.chars().filter(|c| *c == '2').count();

    let result = layer_ones * layer_twos;

    println!(
        "On the layer that contains the fewest 0 digits, the number of 1 digits multiplied by the number of 2 digits: {}",
        result
    );
}
