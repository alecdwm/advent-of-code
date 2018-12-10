//! You've sneaked into another supply closet - this time, it's across from the prototype suit manufacturing lab. You need to sneak inside and fix the issues with the suit, but there's a guard stationed outside the lab, so this is as close as you can safely get.

use std::collections::BTreeMap;

/// As you search the closet for anything that might help, you discover that you're not the first person to want to sneak in. Covering the walls, someone has spent an hour starting every midnight for the past few months secretly observing this guard post! They've been writing down the ID of the one guard on duty that night - the Elves seem to have decided that one guard was enough for the overnight shift - as well as when they fall asleep or wake up while at their post (your puzzle input).
///
/// For example, consider the following records, which have already been organized into chronological order:
///
/// [1518-11-01 00:00] Guard #10 begins shift
/// [1518-11-01 00:05] falls asleep
/// [1518-11-01 00:25] wakes up
/// [1518-11-01 00:30] falls asleep
/// [1518-11-01 00:55] wakes up
/// [1518-11-01 23:58] Guard #99 begins shift
/// [1518-11-02 00:40] falls asleep
/// [1518-11-02 00:50] wakes up
/// [1518-11-03 00:05] Guard #10 begins shift
/// [1518-11-03 00:24] falls asleep
/// [1518-11-03 00:29] wakes up
/// [1518-11-04 00:02] Guard #99 begins shift
/// [1518-11-04 00:36] falls asleep
/// [1518-11-04 00:46] wakes up
/// [1518-11-05 00:03] Guard #99 begins shift
/// [1518-11-05 00:45] falls asleep
/// [1518-11-05 00:55] wakes up
///
/// Timestamps are written using year-month-day hour:minute format. The guard falling asleep or waking up is always the one whose shift most recently started. Because all asleep/awake times are during the midnight hour (00:00 - 00:59), only the minute portion (00 - 59) is relevant for those events.
///
/// Visually, these records show that the guards are asleep at these times:
///
/// Date   ID   Minute
///             000000000011111111112222222222333333333344444444445555555555
///             012345678901234567890123456789012345678901234567890123456789
/// 11-01  #10  .....####################.....#########################.....
/// 11-02  #99  ........................................##########..........
/// 11-03  #10  ........................#####...............................
/// 11-04  #99  ....................................##########..............
/// 11-05  #99  .............................................##########.....
///
/// The columns are Date, which shows the month-day portion of the relevant day; ID, which shows the guard on duty that day; and Minute, which shows the minutes during which the guard was asleep within the midnight hour. (The Minute column's header shows the minute's ten's digit in the first row and the one's digit in the second row.) Awake is shown as ., and asleep is shown as #.
///
/// Note that guards count as asleep on the minute they fall asleep, and they count as awake on the minute they wake up. For example, because Guard #10 wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
///
/// If you can figure out the guard most likely to be asleep at a specific time, you might be able to trick that guard into working tonight so you can have the best chance of sneaking in. You have two strategies for choosing the best guard/minute combination.
///
/// Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?
///
/// In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas any other minute the guard was asleep was only seen on one day).
///
/// While this example listed the entries in chronological order, your entries are in the order you found them. You'll need to organize them before they can be analyzed.
///
/// What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 10 * 24 = 240.)
pub fn part1() {
    let input = crate::common::read_stdin_to_string();

    let mut sorted_input: Vec<_> = input.lines().collect();
    sorted_input.sort();

    let sleep_schedule = build_part_1_sleep_schedule(sorted_input);

    let most_slept_guard = &sleep_schedule
        .iter()
        .fold((0, 0), |most_slept, (k, v)| {
            if most_slept.0 < v.0 {
                (v.0, *k)
            } else {
                most_slept
            }
        })
        .1;

    let mut slept_minutes: BTreeMap<i64, i64> = BTreeMap::new();
    for minute in &sleep_schedule.get(&most_slept_guard).unwrap().1 {
        *slept_minutes.entry(*minute).or_insert(0) += 1;
    }

    let most_slept_minute = slept_minutes
        .iter()
        .fold((0, 0), |most_slept, (k, v)| {
            if most_slept.1 < *v {
                (*k, *v)
            } else {
                most_slept
            }
        })
        .0;

    println!(
        "the id of the guard ({}) multiplied by the minute ({}): {}",
        most_slept_guard,
        most_slept_minute,
        most_slept_guard * most_slept_minute
    );
}

/// Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
///
/// In the example above, Guard #99 spent minute 45 asleep more than any other guard or minute - three times in total. (In all other cases, any guard spent any minute asleep at most twice.)
///
/// What is the ID of the guard you chose multiplied by the minute you chose? (In the above example, the answer would be 99 * 45 = 4455.)
pub fn part2() {
    let input = crate::common::read_stdin_to_string();

    let mut sorted_input: Vec<_> = input.lines().collect();
    sorted_input.sort();

    let sleep_schedule = build_part_2_sleep_schedule(sorted_input);

    let most_slept_count_and_minute_and_guard =
        &sleep_schedule
            .iter()
            .fold((0, 0, 0), |most_slept, (guard, minutes)| {
                minutes
                    .iter()
                    .fold(most_slept, |most_slept, (minute, slept_count)| {
                        if most_slept.0 < *slept_count {
                            (*slept_count, *minute, *guard)
                        } else {
                            most_slept
                        }
                    })
            });
    let most_slept_minute = most_slept_count_and_minute_and_guard.1;
    let most_slept_guard = most_slept_count_and_minute_and_guard.2;

    println!(
        "the id of the guard ({}) multiplied by the minute ({}): {}",
        most_slept_guard,
        most_slept_minute,
        most_slept_guard * most_slept_minute
    );
}

fn build_part_1_sleep_schedule<'a, T: IntoIterator<Item = &'a str>>(
    sorted_input: T,
) -> BTreeMap<i64, (i64, Vec<i64>)> {
    let mut sleep_schedule = BTreeMap::new();
    let mut guard = 0;
    let mut last_minute = 0;

    for line in sorted_input {
        let minute = line
            .chars()
            .skip(15)
            .take(2)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        match line.split(' ').skip(2).next().unwrap() {
            "Guard" => {
                guard = line
                    .split(' ')
                    .skip(3)
                    .next()
                    .unwrap()
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
            }
            "falls" => {
                last_minute = minute;
            }
            "wakes" => {
                let entry = sleep_schedule.entry(guard).or_insert((0, Vec::new()));
                let mut minutes_slept = minute - last_minute;
                while minutes_slept < 0 {
                    minutes_slept = 60 - minutes_slept;
                }
                entry.0 += minutes_slept;
                for min in last_minute..(last_minute + minutes_slept) {
                    entry.1.push(min % 60);
                }
                last_minute = minute;
            }
            _ => panic!("unhandled input: {}", line),
        }
    }

    sleep_schedule
}

fn build_part_2_sleep_schedule<'a, T: IntoIterator<Item = &'a str>>(
    sorted_input: T,
) -> BTreeMap<i64, BTreeMap<i64, i64>> {
    let mut sleep_schedule = BTreeMap::new();
    let mut guard = 0;
    let mut last_minute = 0;

    for line in sorted_input {
        let minute = line
            .chars()
            .skip(15)
            .take(2)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        match line.split(' ').skip(2).next().unwrap() {
            "Guard" => {
                guard = line
                    .split(' ')
                    .skip(3)
                    .next()
                    .unwrap()
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
            }
            "falls" => {
                last_minute = minute;
            }
            "wakes" => {
                let guard_entry = sleep_schedule.entry(guard).or_insert(BTreeMap::new());
                let mut minutes_slept = minute - last_minute;
                while minutes_slept < 0 {
                    minutes_slept = 60 - minutes_slept;
                }

                for min in last_minute..(last_minute + minutes_slept) {
                    *guard_entry.entry(min % 60).or_insert(0) += 1
                }

                last_minute = minute;
            }
            _ => panic!("unhandled input: {}", line),
        }
    }

    sleep_schedule
}
