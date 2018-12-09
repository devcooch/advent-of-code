use itertools::Itertools;

fn main() {
    let contents = include_str!("day02.txt");
    for (line1, line2) in contents.lines().cartesian_product(contents.lines()) {
        let diff: i32 = line1
            .chars()
            .zip(line2.chars())
            .map(|c| (c.0 != c.1) as i32)
            .sum();
        if diff == 1 {
            let word: String = line1
                .chars()
                .zip(line2.chars())
                .filter(|c| (c.0 == c.1))
                .map(|c| c.0)
                .collect();
            println!("{}", word);
            break;
        }
    }
}
