extern crate advent_of_code_2018;

use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: advent-of-code-2018 list | <puzzle_solution>");
        std::process::exit(1);
    }

    let mut puzzle_solution_map = BTreeMap::new();
    puzzle_solution_map.insert("day1::part1", advent_of_code_2018::day1::part1);

    let command = args[1].as_str();
    if command == "list" {
        for puzzle_solution in puzzle_solution_map.keys() {
            println!("{}", puzzle_solution);
        }
        std::process::exit(1);
    }

    let puzzle_solution = args[1].as_str();
    match puzzle_solution_map.get(puzzle_solution) {
        None => {
            eprintln!("puzzle solution '{}' not found", puzzle_solution);
            std::process::exit(1);
        }
        Some(day_fn) => day_fn(),
    }
}
