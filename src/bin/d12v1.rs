use std::collections::HashMap;
use std::str;

fn main() {
    let data = include_str!("day12.txt");
    let mut lines = data.lines();
    let mut state: Vec<u8> = lines.next().unwrap().trim().as_bytes().to_vec();
    let zero_pos = state.iter().position(|&x| x == b'#' || x == b'.').unwrap();
    state = state
        .iter()
        .map(|x| match x {
            b'#' => b'#',
            b'.' => b'.',
            _ => b'.',
        })
        .collect();
    state.append(&mut vec![b'.'; 30]);
    lines.next();
    println!("{}", str::from_utf8(&state).unwrap());
    let mut rules: HashMap<Vec<u8>, u8> = HashMap::new();
    for line in lines {
        let mut ls = line.trim().split(" => ");
        let from = ls.next().unwrap().as_bytes().to_vec();
        let to = ls.next().unwrap().as_bytes().to_vec()[0];
        rules.insert(from, to);
    }
    for _ in 0..20 {
        let mut new_state = vec![b'.'; state.len()];
        for pos in 2..state.len() - 2 {
            let pattern = &state[pos - 2..pos + 3];
            new_state[pos] = rules[pattern];
        }
        state = new_state;
    }
    let result: i32 = state
        .iter()
        .enumerate()
        .map(|(i, x)| match x {
            b'#' => (i - zero_pos) as i32,
            b'.' => 0i32,
            _ => panic!("???"),
        })
        .sum();
    println!("{}", result);
}
