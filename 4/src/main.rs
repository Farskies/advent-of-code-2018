extern crate regex;
extern crate chrono;

use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;
use regex::Regex;
use std::env;
use std::collections::HashMap;

#[derive(Debug)]
enum Event {
    WakeUp,
    FallAsleep,
    BeginShift(u32)
}

#[derive(Debug)]
struct LogEntry {
    date: DateTime<Utc>,
    event: Event
}

#[derive(Debug)]
struct MostFrequentMinute {
    guard_id: u32,
    minute: u32,
    frequency: u32
}

#[derive(Debug)]
struct MostAsleepGuard {
    guard_id: u32,
    num_minutes: u32,
    most_asleep_minute: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = get_input(&args[1]);

    let log = parse_log(&contents);

    let sleep_map = calc_sleep_map(&log);

    println!("{:?}", most_asleep_guard(&sleep_map));

    println!("{:?}", most_frequent_minute(&sleep_map));
}

fn most_asleep_guard(sleep_log: &HashMap<u32, Vec<u32>>) -> MostAsleepGuard {
    let mut candidate = MostAsleepGuard {
        guard_id: 0,
        num_minutes: 0,
        most_asleep_minute: 0
    };

    for (guard_id, sleep_map) in sleep_log.iter() {
        let mut num_minutes = 0u32;
        let mut most_asleep_minute = 0u32;
        let mut most_asleep_minute_minutes = 0u32;

        for (minute, minutes_asleep) in sleep_map.iter().enumerate() {
            num_minutes += *minutes_asleep;

            if *minutes_asleep > most_asleep_minute_minutes {
                most_asleep_minute_minutes = *minutes_asleep;
                most_asleep_minute = minute as u32; 
            }
        }


        if num_minutes > candidate.num_minutes {
            candidate.num_minutes = num_minutes;
            candidate.guard_id = *guard_id;
            candidate.most_asleep_minute = most_asleep_minute;
        }
    }

    candidate
}

fn most_frequent_minute(sleep_log: &HashMap<u32, Vec<u32>>) -> MostFrequentMinute {
    let mut candidate = MostFrequentMinute {
        minute: 0,
        frequency: 0,
        guard_id: 0
    };

    for i in 0..60 { 
        let mut frequency_guard_id = 0u32;
        let mut frequency = 0u32;

        for (guard_id, sleep_map) in sleep_log.iter() {
            let guard_frequency = sleep_map.get(i).unwrap();

            if *guard_frequency > frequency {
                frequency = *guard_frequency;
                frequency_guard_id = *guard_id;
            }
        }

        if frequency > candidate.frequency {
            candidate.frequency = frequency;
            candidate.guard_id = frequency_guard_id;
            candidate.minute = i as u32;
        }
    } 
    

    candidate
}

fn get_input(filename: &str) -> String {
    let mut f = File::open(filename)
        .expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("failed to read file");

    contents
}

fn parse_log(input: &str) -> Vec<LogEntry> {
    let regex = Regex::new(r"(?m)^\[(\d+-\d\d-\d\d\s\d\d:\d\d)\]\s(.+)$").unwrap();

    let mut log: Vec<LogEntry> = regex.captures_iter(input)
        .map(|log_entry| {
            let date = Utc.datetime_from_str(&log_entry[1], "%Y-%m-%d %H:%M").unwrap();

            let event = match &log_entry[2] {
                "wakes up" => Event::WakeUp,
                "falls asleep" => Event::FallAsleep,
                _ => Event::BeginShift(parse_guard_id(&log_entry[2]))
            };

            LogEntry {
                date,
                event
            }
        })
        .collect();

    log.sort_unstable_by(|a, b| a.date.cmp(&b.date));

    log
}

fn parse_guard_id(input: &str) -> u32 {
    let regex = Regex::new(r"(\d+)").unwrap();

    regex.find(input).unwrap().as_str().parse::<u32>().unwrap()
}

fn calc_sleep_map(log: &Vec<LogEntry>) -> HashMap<u32, Vec<u32>> {
    let mut map = HashMap::new();
    let mut current_guard_id: u32 = 0;
    let mut asleep_at: Option<DateTime<Utc>> = None;

    for entry in log {
        match entry.event {
            Event::BeginShift(guard_id) => current_guard_id = guard_id,
            Event::FallAsleep => {
                asleep_at = Some(entry.date);
            },
            Event::WakeUp => {
                let duration_asleep = entry.date.signed_duration_since(asleep_at.unwrap());
                let mut guard_sleep_log = map.entry(current_guard_id).or_insert(vec![0; 60]);

                for i in 0..(duration_asleep.num_minutes() as u32) {
                    *guard_sleep_log.get_mut((i + asleep_at.unwrap().minute()) as usize).unwrap() += 1;
                }
            }
        }
    }

    map
}
