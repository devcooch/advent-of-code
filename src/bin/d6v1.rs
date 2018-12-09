fn main() {
    let data = include_str!("day06.txt");
    let input: Vec<Vec<_>> = data
        .lines()
        .map(|x| {
            x.split(", ")
                .into_iter()
                .map(|y| y.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
}
