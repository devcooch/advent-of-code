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
    for point in convex_hull(&mut input) {
        println!("{}", point);
    }
}
