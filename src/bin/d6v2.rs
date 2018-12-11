#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn distance(p1: Point, p2: Point) -> u32 {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as u32
}

fn main() {
    let data = include_str!("day06.txt");
    let input: Vec<Point> = data
        .lines()
        .map(|line| line.split(", ").map(|item| item.parse().unwrap()).collect())
        .map(|item: Vec<i32>| Point {
            x: item[0],
            y: item[1],
        })
        .collect();
    let left = input.iter().map(|p| p.x).min().unwrap() as usize;
    let right = input.iter().map(|p| p.x).max().unwrap() as usize;
    let top = input.iter().map(|p| p.y).max().unwrap() as usize;
    let bottom = input.iter().map(|p| p.y).min().unwrap() as usize;
    let mut total = 0;
    for x in left..right {
        for y in bottom..top {
            let candidate = Point {
                x: x as i32,
                y: y as i32,
            };
            let distance_sum: u32 = input.iter().map(|&p| distance(p, candidate)).sum();
            if distance_sum < 10_000 {
                total += 1
            }
        }
    }
    println!("{}", total);
}
