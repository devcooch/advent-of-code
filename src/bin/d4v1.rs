use regex::Regex;
use std::collections::{BTreeMap, HashMap};

enum Status {
    Start,
    Asleep,
    Awake,
}

struct Event {
    pub status: Status,
    pub status_text: String,
    pub id: Option<usize>,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
}

fn status_to_enum(status: &str) -> Status {
    match status.get(0..1).unwrap() {
        "G" => Status::Start,
        "f" => Status::Asleep,
        "w" => Status::Awake,
        _ => panic!("Unexpected character {}", status),
    }
}

fn days_in_month(month: usize) -> usize {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => panic!("Unexpected month {}", month),
    }
}

fn minutes_since_year_start(month: usize, day: usize, hour: usize, minute: usize) -> usize {
    let mut i = month - 1;
    let mut result: usize = 0;
    while i > 0 {
        result += days_in_month(i);
        i -= 1;
    }
    result += day;
    result *= 24;
    result += hour;
    result *= 60;
    result += minute;
    result
}

fn main() {
    let data = include_str!("day04.txt");
    let re = Regex::new(r"\[\d+-(?P<month>\d+)-(?P<day>\d+) (?P<hour>\d+):(?P<minute>\d+)\] (?P<status>Guard #(?P<id>\d+) begins shift|falls asleep|wakes up)").unwrap();
    let mut events = BTreeMap::new();
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let month: usize = caps.name("month").unwrap().as_str().parse().unwrap();
        let day: usize = caps.name("day").unwrap().as_str().parse().unwrap();
        let hour: usize = caps.name("hour").unwrap().as_str().parse().unwrap();
        let minute: usize = caps.name("minute").unwrap().as_str().parse().unwrap();
        let time = minutes_since_year_start(month, day, hour, minute);
        let status = caps.name("status").unwrap().as_str();
        let event = Event {
            status: status_to_enum(status),
            status_text: status.to_string(),
            id: match caps.name("id") {
                Some(_) => Some(caps.name("id").unwrap().as_str().parse().unwrap()),
                None => None,
            },
            month: month,
            day: day,
            hour: hour,
            minute: minute,
        };
        events.insert(time, event);
    }
    let mut sleeps = HashMap::new();
    let mut total = HashMap::new();
    let mut id = 0;
    let mut from: usize = 0;
    for event in events.values() {
        match event.status {
            Status::Start => {
                id = event.id.unwrap();
            }
            Status::Asleep => {
                from = event.minute;
            }
            Status::Awake => {
                let entry = sleeps.entry(id).or_insert(vec![0_usize; 60]);
                for x in entry.iter_mut().skip(from).take(event.minute) {
                    *x += 1;
                }
                *total.entry(id).or_insert(0) += event.minute - from + 1;
            }
        }
    }
    let most = total.iter().max_by_key(|x| x.1).unwrap();
    let on_minute = sleeps.get(most.0).unwrap().iter().max().unwrap();
    println!("#{} total: {} min, worst {} min", most.0, most.1, on_minute);
    println!("Answer is {}", on_minute * most.0);
}
