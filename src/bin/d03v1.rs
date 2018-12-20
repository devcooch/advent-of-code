use regex::Regex;

fn main() {
    let data = include_str!("day03.txt");
    let mut patch = vec![vec![0; 1000]; 1000];
    let re = Regex::new(r"#\d+ @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let x = caps.name("x").unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.name("y").unwrap().as_str().parse::<usize>().unwrap();
        let w = caps.name("w").unwrap().as_str().parse::<usize>().unwrap();
        let h = caps.name("h").unwrap().as_str().parse::<usize>().unwrap();
        for row in patch.iter_mut().skip(x).take(w) {
            for item in row.iter_mut().skip(y).take(h) {
                *item += 1;
            }
        }
    }
    let mut total = 0;
    for row in patch.iter() {
        for item in row.iter() {
            if *item > 1 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}
