use std::collections::HashMap;

fn main() {
    let contents = include_str!("day02.txt");
    let mut twos = 0;
    let mut threes = 0;
    for line in contents.lines() {
        let mut chars = HashMap::new();
        for c in line.chars() {
            *chars.entry(c).or_insert(0) += 1;
        }
        if chars.values().any(|&x| x == 2) {
            twos += 1;
        }
        if chars.values().any(|&x| x == 3) {
            threes += 1;
        }
    }
    println!("{}", twos * threes);
}
