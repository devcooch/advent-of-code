use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};

fn duration(c: char) -> u8 {
    (c as u8) - ('A' as u8) + 61
}

fn main() {
    let data = include_str!("day07.txt");
    let mut parents = HashMap::new();
    let mut childs = HashMap::new();
    let mut durations = HashMap::new();
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let parent = &caps[1];
        let child = &caps[2];
        parents
            .entry(child.to_string())
            .or_insert(HashSet::new())
            .insert(parent.to_string());
        parents.entry(parent.to_string()).or_insert(HashSet::new());
        childs
            .entry(parent.to_string())
            .or_insert(HashSet::new())
            .insert(child.to_string());
        childs.entry(child.to_string()).or_insert(HashSet::new());
        durations
            .entry(parent.to_string())
            .or_insert(duration(parent.chars().next().unwrap()));
        durations
            .entry(child.to_string())
            .or_insert(duration(child.chars().next().unwrap()));
    }
    let mut candidates: Vec<String> = parents
        .iter()
        .filter(|(_, value)| value.is_empty())
        .map(|(key, _)| key.to_string())
        .collect();
    let mut visited = HashSet::new();
    let mut total: u32 = 0;
    while !candidates.is_empty() {
        candidates.sort_by(|a, b| b.cmp(a));
        candidates.dedup();
        println!("{}", candidates.join(""));
        let count = cmp::min(4, candidates.len());
        let (index, min_duration) = candidates[0..count]
            .iter()
            .map(|c| durations[c])
            .enumerate()
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap();
        println!("{} {}", index, min_duration);
        total += min_duration as u32;
        for i in 0..count {
            let borrowed = candidates[i].to_string();
            durations.entry(borrowed).and_modify(|e| *e -= min_duration);
        }
        let candidate = candidates.remove(index);
        print!("{}", candidate);
        for child in &childs[&candidate] {
            //println!("{} -> {}", child, parents[child].iter().cloned().collect::<Vec<String>>().join(""));
            if !visited.contains(child)
                && parents[child]
                    .iter()
                    .all(|p| visited.contains(p) || p == &candidate)
            {
                candidates.push(child.to_string());
            }
        }
        visited.insert(candidate);
    }
    println!("{}", total);
}
