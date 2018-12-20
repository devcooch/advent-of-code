use regex::Regex;
use std::collections::{HashMap,HashSet};

fn main() {
    let data = include_str!("day07.txt");
    let mut parents = HashMap::new();
    let mut childs = HashMap::new();
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let parent = &caps[1];
        let child = &caps[2];
        parents.entry(child.to_string()).or_insert(HashSet::new()).insert(parent.to_string());
        parents.entry(parent.to_string()).or_insert(HashSet::new());
        childs.entry(parent.to_string()).or_insert(HashSet::new()).insert(child.to_string());
        childs.entry(child.to_string()).or_insert(HashSet::new());
    }
    let mut candidates: Vec<String> = parents.iter().filter(| (_, value) | value.is_empty()).map(|(key, _)| key.to_string()).collect();
    let mut visited = HashSet::new();
    while !candidates.is_empty() {
        let candidate = candidates.pop().unwrap();
        print!("{}", candidate);
        for child in &childs[&candidate] {
            //println!("{} -> {}", child, parents[child].iter().cloned().collect::<Vec<String>>().join(""));
            if !visited.contains(child) && parents[child].iter().all(|p| visited.contains(p) || p == &candidate) {
                candidates.push(child.to_string());
            }
        }
        candidates.sort_by(|a,b| b.cmp(a));
        candidates.dedup();
        //println!("{}", candidates.join(""));
        visited.insert(candidate);
    }
}
