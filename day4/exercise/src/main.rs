extern crate chrono;
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use chrono::prelude::*;
use std::cmp::PartialOrd;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;

fn get_input_data() -> Vec<String> {
    let file_path = args().nth(1).expect("Input file path is required");
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    return reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect();
}

#[derive(PartialEq, Eq, Debug)]
enum GuardAction {
    WakeUp,
    FallAsleep,
    StartShift(u16),
}

impl From<String> for GuardAction {
    fn from(input: String) -> GuardAction {
        lazy_static! {
            static ref matcher: Regex = Regex::new(r"(?:Guard #)?(\d*)\s*(.*)").unwrap();
        }
        let matched = matcher.captures_iter(&input).next().unwrap();

        let guard = match matched[1].parse::<u16>() {
            Ok(id) => Some(id),
            Err(_) => None
        };

        return match &matched[2] {
            "begins shift" => GuardAction::StartShift(guard.unwrap()),
            "falls asleep" => GuardAction::FallAsleep,
            "wakes up" => GuardAction::WakeUp,
            _ => panic!("Could not create GuardAction"),
        };
    }
}

#[derive(Debug, Eq)]
struct Record {
    date: DateTime<Utc>,
    action: GuardAction,
}

impl From<String> for Record {
    fn from (input: String) -> Record {
        lazy_static! {
            static ref matcher: Regex = Regex::new(r"\[(.*?)\]\s*(.*)").unwrap();
        }
        let matched = matcher.captures_iter(&input).next().unwrap();

        let date = Utc.datetime_from_str(&matched[1], "%Y-%m-%d %H:%M").expect("Could not parse date");

        let action = GuardAction::from(matched[2].to_string());

        return Record {
            date: date,
            action: action,
        };
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        return self.date.cmp(&other.date);
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        return self.date == other.date;
    }
}

struct Schedule {
    minutes_asleep_by_guard: HashMap<u16, [i32 ; 60]>
}

impl Schedule {
    fn new(records: &Vec<Record>) -> Schedule {
        let mut minutes_asleep_by_guard: HashMap<u16, [i32; 60]> = HashMap::new();
        let mut current_guard: Option<u16> = None;
        let mut began_sleeping_at: Option<DateTime<Utc>> = None;

        for record in records {
            match record.action {
                GuardAction::StartShift(guard_id) => {
                    if !minutes_asleep_by_guard.contains_key(&guard_id) {
                        minutes_asleep_by_guard.insert(guard_id, [0 ; 60]);
                    }

                    current_guard = Some(guard_id);
                    began_sleeping_at = None;
                }
                GuardAction::FallAsleep => {
                    began_sleeping_at = Some(record.date);
                }
                GuardAction::WakeUp => {
                    if let (Some(current_guard), Some(began_sleeping_at)) = (current_guard, began_sleeping_at) {
                        let minutes_slept = record.date.signed_duration_since(began_sleeping_at).num_minutes();
                        let mut minutes_for_guard: [i32; 60] = minutes_asleep_by_guard.get(&current_guard).unwrap().clone();

                        for minute in 0..minutes_slept {
                            let minute_to_set: usize = ((began_sleeping_at.minute() + (minute as u32)) % 60) as usize;
                            minutes_for_guard[minute_to_set] += 1;
                        }
                        minutes_asleep_by_guard.insert(current_guard, minutes_for_guard);
                    }
                }
            }
        }
        return Schedule { minutes_asleep_by_guard: minutes_asleep_by_guard }
    }

    fn find_sleepiest_guard(&self) -> (u16, i32) {
        let mut max = 0;
        let mut max_guard = 0;
        for (guard, values) in &self.minutes_asleep_by_guard {
            let sleep_minutes: i32 = values.iter().fold(0, |acc, value| acc + value);
            if sleep_minutes > max {
                max_guard = guard.clone();
                max = sleep_minutes;
            }
        }

        return (max_guard, max);
    }

    fn find_most_consistent_sleep_guard(&self) -> (u16, i32) {
        let mut max = 0;
        let mut max_guard = 0;
        let mut max_freq_index = 0;

        for (&guard, values) in &self.minutes_asleep_by_guard {
            for (index, &freq) in values.iter().enumerate() {
                if freq > max {
                    max = freq;
                    max_guard = guard;
                    max_freq_index = index;
                }
            }
        }

        return (max_guard, max_freq_index as i32);
    }

    fn find_sleepiest_minute_by_guard(&self, guard: u16) -> i32 {
        let mut max = 0;
        let mut max_index = 0;

        let sleep_minutes = self.minutes_asleep_by_guard.get(&guard).unwrap();

        for i in 0..sleep_minutes.len() {
            if sleep_minutes[i] > max {
                max = sleep_minutes[i];
                max_index = i;
            }
        }

        return max_index as i32;
    }
}

fn main() {
    let record_input = get_input_data();

    let mut records: Vec<Record> = record_input.into_iter()
        .map(Record::from)
        .collect();

    records.sort();

    let schedule = Schedule::new(&records);
    let (sleepiest_guard, _) = schedule.find_sleepiest_guard();
    let sleepiest_minute = schedule.find_sleepiest_minute_by_guard(sleepiest_guard);

    let (most_frequent_guard, highest_frequence_minute) = schedule.find_most_consistent_sleep_guard();

    println!("By sleep minutes {}", sleepiest_guard as i32 * sleepiest_minute);
    println!("By sleep frequency {}", most_frequent_guard as i32 * highest_frequence_minute);
}

#[cfg(test)]
mod test {
    use Record;
    use GuardAction;
    use chrono::prelude::*;
    use std::cmp::Ordering;

    #[test]
    fn test_wake_up_record() {
        let result = Record::from("[1518-09-14 00:54] wakes up".to_string());
        let expected = Record {
            date: Utc.ymd(1518, 9, 14).and_hms(0, 54, 0),
            action: GuardAction::WakeUp,
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_fall_asleep_record() {
        let result = Record::from("[1518-09-14 00:54] falls asleep".to_string());
        let expected = Record {
            date: Utc.ymd(1518, 9, 14).and_hms(0, 54, 0),
            action: GuardAction::FallAsleep,
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_shift_starts_record() {
        let result = Record::from("[1518-04-15 23:58] Guard #373 begins shift".to_string());
        let expected = Record {
            date: Utc.ymd(1518, 4, 15).and_hms(23, 58, 0),
            action: GuardAction::StartShift(373),
        };

        assert_eq!(expected, result);
    }

    #[test]
    fn test_sort_record() {
        let first = Record {
            date: Utc.ymd(1518, 4, 15).and_hms(23, 58, 0),
            action: GuardAction::StartShift(373),
        };
        let second = Record {
            date: Utc.ymd(1518, 9, 14).and_hms(0, 54, 0),
            action: GuardAction::WakeUp,
        };

        assert_eq!(second.cmp(&first), Ordering::Greater);
        assert_eq!(first.cmp(&second), Ordering::Less);
        assert_eq!(first.cmp(&first), Ordering::Equal);
    }
}
