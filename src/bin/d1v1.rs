fn main() {
    let contents = include_str!("day01.txt");
    println!(
        "{}",
        contents
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .sum::<i32>()
    );
}
