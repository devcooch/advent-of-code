use std::collections::HashMap;

fn main() {
    let data = include_str!("day12.txt");
    let mut lines = data.lines();
    let first_line = lines.next().unwrap().trim();
    let mut state: Vec<u8> = first_line.as_bytes().to_vec();
    let zero_pos = state.iter().position(|&x| x == b'#' || x == b'.').unwrap();
    state = state
        .iter()
        .map(|x| match x {
            b'#' => b'#',
            b'.' => b'.',
            _ => b'.',
        })
        .collect();
    state.append(&mut vec![b'.'; 1000]);
    lines.next();
    println!("{}", first_line);
    let mut rules: HashMap<Vec<u8>, u8> = HashMap::new();
    for line in lines {
        let mut ls = line.trim().split(" => ");
        let from = ls.next().unwrap().as_bytes().to_vec();
        let to = ls.next().unwrap().as_bytes().to_vec()[0];
        rules.insert(from, to);
    }
    let mut patterns: Vec<String> = Vec::new();
    let mut states: Vec<Vec<u8>> = Vec::new();
    for cycle in 0..1000 {
        let mut pattern = String::from_utf8(state.clone()).unwrap();
        let first_hash = pattern.find('#').unwrap();
        pattern = pattern.split_at(first_hash).1.to_string();
        let last_hash = pattern.rfind('#').unwrap();
        pattern = pattern.split_at(last_hash + 1).0.to_string();
        let index = patterns.iter().rposition(|item| item == &pattern);
        if index.is_some() {
            let offset = index.unwrap();
            let speed = first_hash - states[offset].iter().position(|&b| b == b'#').unwrap();
            println!(
                "Cycle: {}, index:{}, speed: {}, starts: {}",
                cycle, offset, speed, first_hash
            );
            let final_first_hash: u64 = 50_000_000_000_u64 - cycle + first_hash as u64;
            let result: u64 = pattern
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(i, x)| match x {
                    b'#' => i as u64 + final_first_hash - zero_pos as u64,
                    b'.' => 0u64,
                    _ => panic!("???"),
                })
                .sum();
            println!("{}", result);
            break;
        }
        patterns.push(pattern);
        let mut new_state = vec![b'.'; state.len()];
        for pos in 2..new_state.len() - 2 {
            let pattern = &state[pos - 2..pos + 3];
            new_state[pos] = rules[pattern];
        }
        states.push(state);
        state = new_state;
    }
}
