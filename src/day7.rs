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
    while let Some(step) = step_graph.next_step() {
        steps.push(step);
    }

    println!(
        "the order the steps should be completed: {}",
        steps.iter().collect::<String>()
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

    fn next_step(&mut self) -> Option<char> {
        if self.available_steps.len() < 1 {
            return None;
        }

        self.available_steps.sort_unstable_by(|a, b| b.cmp(a));

        let next_step = match self.available_steps.pop() {
            Some(step) => step,
            None => return None,
        };
        self.completed_steps.push(next_step);

        let mut new_available_steps = Vec::new();
        for (step, dependencies) in &self.step_dependencies {
            if self.completed_steps.contains(step) || self.available_steps.contains(step) {
                continue;
            }
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

        Some(next_step)
    }
}
