use std::collections::HashSet;

fn main() {
    let contents = include_str!("day01.txt");
    let mut freq = 0;
    let mut freqs = HashSet::new();
    let mut line = contents.lines().cycle();
    while freqs.insert(freq) {
        freq += line.next().unwrap().parse::<i32>().unwrap();
    }
    println!("{}", freq);
}
