fn collapse_word(word: &[&u8]) -> usize {
    let mut result = Vec::new();
    result.reserve(word.len());
    let mut last_char: u8 = 0;
    for c in word.iter() {
        if (i32::from(**c) - i32::from(last_char)).abs() == 32 {
            result.pop();
            last_char = *result.last().unwrap_or(&0);
        } else {
            last_char = **c;
            result.push(**c);
        }
    }
    result.len()
}

fn main() {
    let word = include_bytes!("day05.txt");
    let mut nums = Vec::new();
    for b in 65_u8..91_u8 {
        let current_word = word
            .into_iter()
            .filter(|x| **x != b && **x != (b + 32))
            .collect::<Vec<_>>();
        nums.push(collapse_word(&current_word));
    }
    println!("{}", nums.iter().min().unwrap());
}
