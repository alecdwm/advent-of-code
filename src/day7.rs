//! You find yourself standing on a snow-covered coastline; apparently, you landed a little off course. The region is too hilly to see the North Pole from here, but you do spot some Elves that seem to be trying to unpack something that washed ashore. It's quite cold out, so you decide to risk creating a paradox by asking them for directions.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

/// "Oh, are you the search party?" Somehow, you can understand whatever Elves from the year 1018 speak; you assume it's Ancient Nordic Elvish. Could the device on your wrist also be a translator? "Those clothes don't look very warm; take this." They hand you a heavy coat.
///
/// "We do need to find our way back to the North Pole, but we have higher priorities at the moment. You see, believe it or not, this box contains something that will solve all of Santa's transportation problems - at least, that's what it looks like from the pictures in the instructions." It doesn't seem like they can read whatever language it's in, but you can: "Sleigh kit. Some assembly required."
///
/// "'Sleigh'? What a wonderful name! You must help us assemble this 'sleigh' at once!" They start excitedly pulling more parts out of the box.
///
/// The instructions specify a series of steps and requirements about which steps must be finished before others can begin (your puzzle input). Each step is designated by a single letter. For example, suppose you have the following instructions:
///
/// Step C must be finished before step A can begin.
/// Step C must be finished before step F can begin.
/// Step A must be finished before step B can begin.
/// Step A must be finished before step D can begin.
/// Step B must be finished before step E can begin.
/// Step D must be finished before step E can begin.
/// Step F must be finished before step E can begin.
///
/// Visually, these requirements look like this:
///
///
///   -->A--->B--
///  /    \      \
/// C      -->D----->E
///  \           /
///   ---->F-----
///
/// Your first goal is to determine the order in which the steps should be completed. If more than one step is ready, choose the step which is first alphabetically. In this example, the steps would be completed as follows:
///
///     Only C is available, and so it is done first.
///     Next, both A and F are available. A is first alphabetically, so it is done next.
///     Then, even though F was available earlier, steps B and D are now also available, and B is the first alphabetically of the three.
///     After that, only D and F are available. E is not available because only some of its prerequisites are complete. Therefore, D is completed next.
///     F is the only choice, so it is done next.
///     Finally, E is completed.
///
/// So, in this example, the correct order is CABDFE.
///
/// In what order should the steps in your instructions be completed?
pub fn part1() {
    let input = crate::common::read_stdin_to_string();

    let instructions = input_to_instructions(input);
    let mut step_graph = StepGraph::from_instructions(instructions);

    let mut steps = Vec::new();
    while let Some(step) = step_graph.next_step(true) {
        steps.push(step);
    }

    println!(
        "the order the steps should be completed: {}",
        steps.iter().collect::<String>()
    );
}

/// As you're about to begin construction, four of the Elves offer to help. "The sun will set soon; it'll go faster if we work together." Now, you need to account for multiple people working on steps simultaneously. If multiple steps are available, workers should still begin them in alphabetical order.
///
/// Each step takes 60 seconds plus an amount corresponding to its letter: A=1, B=2, C=3, and so on. So, step A takes 60+1=61 seconds, while step Z takes 60+26=86 seconds. No time is required between steps.
///
/// To simplify things for the example, however, suppose you only have help from one Elf (a total of two workers) and that each step takes 60 fewer seconds (so that step A takes 1 second and step Z takes 26 seconds). Then, using the same instructions as above, this is how each second would be spent:
///
/// Second   Worker 1   Worker 2   Done
///    0        C          .
///    1        C          .
///    2        C          .
///    3        A          F       C
///    4        B          F       CA
///    5        B          F       CA
///    6        D          F       CAB
///    7        D          F       CAB
///    8        D          F       CAB
///    9        D          .       CABF
///   10        E          .       CABFD
///   11        E          .       CABFD
///   12        E          .       CABFD
///   13        E          .       CABFD
///   14        E          .       CABFD
///   15        .          .       CABFDE
///
/// Each row represents one second of time. The Second column identifies how many seconds have passed as of the beginning of that second. Each worker column shows the step that worker is currently doing (or . if they are idle). The Done column shows completed steps.
///
/// Note that the order of the steps has changed; this is because steps now take time to finish and multiple workers can begin multiple steps simultaneously.
///
/// In this example, it would take 15 seconds for two workers to complete these steps.
///
/// With 5 workers and the 60+ second step durations described above, how long will it take to complete all of the steps?
pub fn part2() {
    let input = crate::common::read_stdin_to_string();

    let instructions = input_to_instructions(input);
    let mut step_graph = StepGraph::from_instructions(instructions);

    let mut seconds = 0;
    let mut gnomes: Vec<(usize, char, usize)> = Vec::new();
    loop {
        while gnomes.len() < 5 {
            let step = match step_graph.next_step(false) {
                Some(step) => step,
                None => break,
            };

            let step_completion_time =
                61 + ALPHABET.iter().position(|letter| *letter == step).unwrap();
            let step_completed_at_time = seconds + step_completion_time;

            let insert_at = match gnomes
                .iter()
                .position(|&gnome| step_completed_at_time > gnome.0)
            {
                Some(position) => position,
                None => gnomes.len(),
            };

            gnomes.insert(
                insert_at,
                (step_completed_at_time, step, step_completion_time),
            );
        }

        let gnome = match gnomes.pop() {
            Some(gnome) => gnome,
            None => break,
        };
        seconds += gnome.2;
        step_graph.complete_step(gnome.1);

        gnomes = gnomes
            .iter()
            .filter_map(|busy_gnome| {
                if busy_gnome.2 < gnome.2 {
                    step_graph.complete_step(busy_gnome.1);
                    return None;
                }
                Some((busy_gnome.0 - gnome.2, busy_gnome.1, busy_gnome.2 - gnome.2))
            })
            .collect();
    }

    println!(
        "seconds it will take to complete all of the steps: {}",
        seconds
    );
}

fn input_to_instructions(input: String) -> Vec<(char, char)> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let split: Vec<_> = line.split(' ').collect();
        instructions.push((split[1].parse().unwrap(), split[7].parse().unwrap()));
    }
    instructions
}

#[derive(Debug)]
struct StepGraph {
    completed_steps: Vec<char>,
    available_steps: Vec<char>,
    step_dependencies: BTreeMap<char, Vec<char>>,
}

impl StepGraph {
    fn new() -> StepGraph {
        StepGraph {
            completed_steps: Vec::new(),
            available_steps: Vec::new(),
            step_dependencies: BTreeMap::new(),
        }
    }

    fn from_instructions(instructions: Vec<(char, char)>) -> StepGraph {
        let mut step_graph = StepGraph::new();
        let mut all_steps = BTreeSet::new();
        for instruction in instructions.iter() {
            all_steps.insert(instruction.0);
            all_steps.insert(instruction.1);
            step_graph.add_step_dependency(instruction.0, instruction.1);
        }
        for step in all_steps.iter() {
            if !step_graph.step_dependencies.contains_key(&step) {
                step_graph.available_steps.push(*step);
            }
        }
        step_graph
    }

    fn add_step_dependency(&mut self, from: char, to: char) {
        self.step_dependencies
            .entry(to)
            .or_insert_with(Vec::new)
            .push(from);
    }

    fn next_step(&mut self, complete_step: bool) -> Option<char> {
        if self.available_steps.len() < 1 {
            return None;
        }

        self.available_steps.sort_unstable_by(|a, b| b.cmp(a));

        let next_step = match self.available_steps.pop() {
            Some(step) => step,
            None => return None,
        };

        if complete_step {
            self.complete_step(next_step);
        }

        Some(next_step)
    }

    fn complete_step(&mut self, step: char) {
        self.completed_steps.push(step);

        let mut new_available_steps = Vec::new();
        for (step, dependencies) in &self.step_dependencies {
            if dependencies
                .iter()
                .all(|step_dependency| self.completed_steps.contains(step_dependency))
            {
                new_available_steps.push(*step);
            }
        }

        for step in new_available_steps {
            self.step_dependencies.remove(&step);
            self.available_steps.push(step);
        }
    }
}

const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', //
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
