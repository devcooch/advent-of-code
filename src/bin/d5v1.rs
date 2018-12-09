fn main() {
    let word = include_bytes!("day05.txt");
    let mut result = Vec::new();
    result.reserve(word.len());
    let mut last_char: u8 = 0;
    for c in word.iter() {
        if (*c as i32 - last_char as i32).abs() == 32 {
            result.pop();
            last_char = *result.last().unwrap_or(&0);
        } else {
            last_char = *c;
            result.push(*c);
        }
    }
    println!("{}", result.len());
}
