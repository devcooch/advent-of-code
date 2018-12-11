use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("Point({},{})", self.x, self.y))
    }
}

fn cross(o: Point, a: Point, b: Point) -> i32 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

fn half_hull(points: &[Point]) -> Vec<Point> {
    let mut result = Vec::with_capacity(points.len());
    for point in points {
        while result.len() >= 2
            && cross(result[result.len() - 2], *result.last().unwrap(), *point) <= 0
        {
            result.pop();
        }
        result.push(*point);
    }
    result.pop();
    result
}

fn convex_hull(points: &mut Vec<Point>) -> Vec<Point> {
    points.sort();
    let mut lower = half_hull(points);
    points.reverse();
    let mut upper = half_hull(points);
    lower.append(&mut upper);
    lower
}

fn distance(p1: Point, p2: Point) -> u32 {
    ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as u32
}

fn main() {
    let data = include_str!("day06.txt");
    let mut input: Vec<Point> = data
        .lines()
        .map(|line| line.split(", ").map(|item| item.parse().unwrap()).collect())
        .map(|item: Vec<i32>| Point {
            x: item[0],
            y: item[1],
        })
        .collect();
    let hull = convex_hull(&mut input);
    let left = hull.iter().map(|p| p.x).min().unwrap() as usize;
    let right = hull.iter().map(|p| p.x).max().unwrap() as usize;
    let top = hull.iter().map(|p| p.y).max().unwrap() as usize;
    let bottom = hull.iter().map(|p| p.y).min().unwrap() as usize;
    println!("{} --- {}, {} ||| {}", left, right, top, bottom);
    println!("{} {}", input.len(), hull.len());
    let height = top - bottom + 1;
    let width = right - left + 1;
    let mut grid = vec![vec![0; height]; width];
    let mut best = HashMap::new();
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, _) in row.iter_mut().enumerate() {
            let candidate = Point {
                x: (i + left) as i32,
                y: (j + bottom) as i32,
            };
            let distances: Vec<_> = input.iter().map(|&p| distance(p, candidate)).collect();
            let min_distance = distances.iter().min().unwrap();
            if distances.iter().filter(|&d| d == min_distance).count() > 1 {
                continue;
            }
            *best
                .entry(distances.iter().position(|d| d == min_distance).unwrap())
                .or_insert(0) += 1;
        }
    }
    println!(
        "{}",
        best.iter()
            .filter(|&(k, _)| !hull.contains(&input[*k]))
            .map(|(_, v)| v)
            .max()
            .unwrap()
    );
}
