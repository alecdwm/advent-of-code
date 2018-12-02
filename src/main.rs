extern crate advent_of_code_2018;

use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: advent-of-code-2018 <day>");
        std::process::exit(1);
    }

    let mut day_fn_map = BTreeMap::new();
    day_fn_map.insert("day1", advent_of_code_2018::day1::day1);

    let day = args[1].as_str();
    match day_fn_map.get(day) {
        None => {
            eprintln!("day '{}' not found", day);
            std::process::exit(1);
        }
        Some(day_fn) => day_fn(),
    }
}
