use std::collections::BTreeMap;
use std::env;

fn main() {
    let mut puzzle_solutions: BTreeMap<&str, fn()> = BTreeMap::new();

    puzzle_solutions.insert("2018::day1::part1", advent_of_code::year_2018::day1::part1);
    puzzle_solutions.insert("2018::day1::part2", advent_of_code::year_2018::day1::part2);
    puzzle_solutions.insert("2018::day2::part1", advent_of_code::year_2018::day2::part1);
    puzzle_solutions.insert("2018::day2::part2", advent_of_code::year_2018::day2::part2);
    puzzle_solutions.insert("2018::day3::part1", advent_of_code::year_2018::day3::part1);
    puzzle_solutions.insert("2018::day3::part2", advent_of_code::year_2018::day3::part2);
    puzzle_solutions.insert("2018::day4::part1", advent_of_code::year_2018::day4::part1);
    puzzle_solutions.insert("2018::day4::part2", advent_of_code::year_2018::day4::part2);
    puzzle_solutions.insert("2018::day5::part1", advent_of_code::year_2018::day5::part1);
    puzzle_solutions.insert("2018::day5::part2", advent_of_code::year_2018::day5::part2);
    puzzle_solutions.insert("2018::day6::part1", advent_of_code::year_2018::day6::part1);
    puzzle_solutions.insert("2018::day6::part2", advent_of_code::year_2018::day6::part2);
    puzzle_solutions.insert("2018::day7::part1", advent_of_code::year_2018::day7::part1);
    puzzle_solutions.insert("2018::day7::part2", advent_of_code::year_2018::day7::part2);

    puzzle_solutions.insert("2019::day1::part1", advent_of_code::year_2019::day1::part1);
    puzzle_solutions.insert("2019::day1::part2", advent_of_code::year_2019::day1::part2);
    puzzle_solutions.insert("2019::day2::part1", advent_of_code::year_2019::day2::part1);
    puzzle_solutions.insert("2019::day2::part2", advent_of_code::year_2019::day2::part2);
    puzzle_solutions.insert("2019::day3::part1", advent_of_code::year_2019::day3::part1);
    puzzle_solutions.insert("2019::day3::part2", advent_of_code::year_2019::day3::part2);
    puzzle_solutions.insert("2019::day4::part1", advent_of_code::year_2019::day4::part1);
    puzzle_solutions.insert("2019::day4::part2", advent_of_code::year_2019::day4::part2);
    puzzle_solutions.insert("2019::day5::part1", advent_of_code::year_2019::day5::part1);
    puzzle_solutions.insert("2019::day5::part2", advent_of_code::year_2019::day5::part2);
    puzzle_solutions.insert("2019::day6::part1", advent_of_code::year_2019::day6::part1);
    puzzle_solutions.insert("2019::day6::part2", advent_of_code::year_2019::day6::part2);
    puzzle_solutions.insert("2019::day7::part1", advent_of_code::year_2019::day7::part1);

    let command = match env::args().nth(1) {
        Some(command) => command,
        None => {
            eprintln!(
                "Usage: advent-of-code <command>\n\nCommands:\n\tlist{}",
                puzzle_solutions
                    .keys()
                    .map(|key| format!("\n\t{}", key))
                    .collect::<String>()
            );
            std::process::exit(1);
        }
    };

    match command.as_str() {
        "list" => {
            for puzzle_solution in puzzle_solutions.keys() {
                println!("{}", puzzle_solution);
            }
            std::process::exit(1);
        }

        puzzle_solution => match puzzle_solutions.get(puzzle_solution) {
            Some(puzzle_solution_fn) => puzzle_solution_fn(),
            None => {
                eprintln!("Puzzle solution '{}' not found", puzzle_solution);
                std::process::exit(1);
            }
        },
    }
}
