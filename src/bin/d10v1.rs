use regex::Regex;

fn main() {
    let data = include_str!("day10.txt");
    let re = Regex::new(
        r"position=<\s*(?P<x>-?\d+),\s+(?P<y>-?\d+)> velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>",
    )
    .unwrap();
    let mut points = Vec::with_capacity(data.lines().count());
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let x: i32 = caps.name("x").unwrap().as_str().parse().unwrap();
        let y: i32 = caps.name("y").unwrap().as_str().parse().unwrap();
        let vx: i32 = caps.name("vx").unwrap().as_str().parse().unwrap();
        let vy: i32 = caps.name("vy").unwrap().as_str().parse().unwrap();
        points.push((x, y, vx, vy));
    }
    for _ in 0..100_000 {
        let min_x = points.iter().map(|p| p.0).min().unwrap();
        let max_x = points.iter().map(|p| p.0).max().unwrap();
        let min_y = points.iter().map(|p| p.1).min().unwrap();
        let max_y = points.iter().map(|p| p.1).max().unwrap();
        if (max_y - min_y) < 20 && (max_x - min_x) < 80 {
            let mut screen = vec![vec![" "; 80]; 20];
            for point in &points {
                screen[(point.1 - min_y) as usize][(point.0 - min_x) as usize] = "*";
            }
            for line in screen {
                println!("{}", line.join(""));
            }
        }
        for mut point in &mut points {
            point.0 += point.2;
            point.1 += point.3;
        }
    }
}
