use regex::Regex;

fn main() {
    let data = include_str!("day03.txt");
    let mut patch = vec![vec![0; 1000]; 1000];
    let mut claims = Vec::new();
    let re = Regex::new(r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)").unwrap();
    for line in data.lines() {
        let caps = re.captures(line).unwrap();
        let id = caps.name("id").unwrap().as_str();
        let x = caps.name("x").unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.name("y").unwrap().as_str().parse::<usize>().unwrap();
        let w = caps.name("w").unwrap().as_str().parse::<usize>().unwrap();
        let h = caps.name("h").unwrap().as_str().parse::<usize>().unwrap();
        for i in x..x + w {
            for j in y..y + h {
                patch[i][j] += 1;
            }
        }
        claims.push((id, x, y, w, h));
    }
    for (id, x, y, w, h) in claims {
        let mut good = true;
        'outer: for i in x..x + w {
            for j in y..y + h {
                if patch[i][j] > 1 {
                    good = false;
                    break 'outer;
                }
            }
        }
        if good {
            println!("{}", id);
        }
    }
}
