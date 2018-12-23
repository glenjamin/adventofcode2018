#[macro_use]
extern crate lazy_static;
extern crate chrono;

use std::collections::HashMap;

use std::io::{self, BufRead};

use chrono::{DateTime, TimeZone, Timelike, Utc};
use regex::Regex;

fn main() {
    let guards: Vec<_> = load_guards(io::stdin().lock());

    let most_asleep_guard =
        most_asleep(&guards).expect("Most asleep not found");
    let peak = most_asleep_guard.peak_minute();

    println!(
        "Most Asleep: #{}\nMinutes: {}\nPeak: {}\nAnswer {}",
        most_asleep_guard.id,
        most_asleep_guard.total_sleep(),
        peak,
        most_asleep_guard.id * peak,
    );

    println!();

    let peak_guard = peak_guard(&guards).expect("Peak guard not found");
    let peak = peak_guard.peak_minute();

    println!(
        "Peak Asleep: #{}\nMinute: {}\nAnswer {}",
        peak_guard.id,
        peak,
        peak_guard.id * peak
    );
}

fn most_asleep(guards: &[Guard]) -> Option<&Guard> {
    guards.iter().max_by_key(|g| g.total_sleep())
}

fn peak_guard(guards: &[Guard]) -> Option<&Guard> {
    guards.iter().max_by_key(|g| g.peak_minute_frequency())
}

fn load_guards(stream: impl BufRead) -> Vec<Guard> {
    let mut lines = stream
        .lines()
        .map(|s| ObservationLine::from(&(s.unwrap())).unwrap())
        .collect::<Vec<ObservationLine>>();

    lines.sort_by_key(|o| o.datetime);

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut current_guard = 0;
    let mut asleep_at = Utc::now();

    let line1 = lines.first().unwrap();
    if let Observation::BeginShift { guard_id } = line1.observation {
        current_guard = guard_id;
        guards
            .entry(guard_id)
            .or_insert_with(|| Guard::new(guard_id));
    }

    for line in lines.iter().skip(1) {
        match line.observation {
            Observation::BeginShift { guard_id } => {
                current_guard = guard_id;
                guards
                    .entry(guard_id)
                    .or_insert_with(|| Guard::new(guard_id));
            }
            Observation::Sleep => asleep_at = line.datetime,
            Observation::Wake => {
                let guard = guards.get_mut(&current_guard).unwrap();
                guard.record_sleep(asleep_at.minute(), line.datetime.minute())
            }
        }
    }

    guards.into_iter().map(|(_, g)| g).collect()
}

#[derive(Debug)]
struct ObservationLine {
    datetime: DateTime<Utc>,
    observation: Observation,
}
#[derive(Debug)]
enum Observation {
    BeginShift { guard_id: u32 },
    Sleep,
    Wake,
}
impl ObservationLine {
    fn from(string: &str) -> Result<ObservationLine, &'static str> {
        lazy_static! {
            static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
        }

        let nums: Vec<u32> = NUM_RE
            .find_iter(&string)
            .flat_map(|m| m.as_str().parse())
            .collect();

        if nums.len() < 5 {
            return Err("Invalid line");
        }

        let datetime = Utc
            .ymd(nums[0] as i32, nums[1], nums[2])
            .and_hms(nums[3], nums[4], 0);

        let observation = if nums.len() == 6 {
            Observation::BeginShift { guard_id: nums[5] }
        } else if string.contains("wakes") {
            Observation::Wake
        } else {
            Observation::Sleep
        };

        Ok(ObservationLine {
            datetime,
            observation,
        })
    }
}

#[derive(Debug)]
struct Guard {
    id: u32,
    sleep_minutes: HashMap<u32, u32>,
}

impl Guard {
    fn new(guard_id: u32) -> Guard {
        Guard {
            id: guard_id,
            sleep_minutes: HashMap::new(),
        }
    }

    fn record_sleep(&mut self, from: u32, to: u32) {
        for min in from..to {
            *self.sleep_minutes.entry(min).or_insert(0) += 1;
        }
    }

    fn total_sleep(&self) -> u32 {
        self.sleep_minutes.values().sum()
    }
    fn peak_minute(&self) -> u32 {
        self.peak_minute_entry().0
    }
    fn peak_minute_frequency(&self) -> u32 {
        self.peak_minute_entry().1
    }
    fn peak_minute_entry(&self) -> (u32, u32) {
        self.sleep_minutes
            .iter()
            .max_by_key(|&(_, v)| v)
            .map(|(&k, &v)| (k, v))
            .unwrap_or((0, 0))
    }
}
