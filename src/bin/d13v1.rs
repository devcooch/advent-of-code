use std::collections::HashSet;

struct Track {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

#[derive(Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Ignore,
}

#[derive(Eq, PartialEq)]
enum Action {
    Left,
    Straight,
    Right,
}

struct Cart {
    x: usize,
    y: usize,
    dir: Dir,
    track: usize,
    action: Action,
}

#[derive(Eq, PartialEq, Hash)]
struct Junction {
    x: usize,
    y: usize,
}

fn process_vertical(from: usize, to: usize, at: usize, map: &mut Vec<Vec<u8>>) {}

fn main() {
    let data = include_bytes!("day13.txt");
    let mut map: Vec<Vec<u8>> = data.split(|&c| c == b'\n').map(|v| v.to_vec()).collect();
    let map_h = map.len();
    let map_w = map[0].len();
    let mut tracks = Vec::new();
    let mut carts = Vec::new();
    let mut junctions = HashSet::new();
    for i in 0..map_h {
        for j in 0..map_w {
            if map[i][j] == b'/' {
                let x = j;
                let y = i;
                let mut w = 0;
                let mut h = 0;
                for k in j + 1..map_w {
                    if map[i][k] == b'\\' {
                        w = k - j + 1;
                        break;
                    }
                }
                for k in i + 1..map_h {
                    if map[k][j] == b'\\' {
                        h = k - i + 1;
                        break;
                    }
                }
                for a in y..y + h {
                    for b in x..x + w {
                        let dir = match map[a][b] {
                            b'>' => Dir::Right,
                            b'<' => Dir::Left,
                            b'v' => Dir::Down,
                            b'^' => Dir::Up,
                            _ => Dir::Ignore,
                        };
                        if dir != Dir::Ignore {
                            let cart = Cart {
                                x: b,
                                y: a,
                                dir,
                                track: tracks.len(),
                                action: Action::Left,
                            };
                            carts.push(cart);
                        }
                        match map[a][b] {
                            b'+' => {
                                map[a][b] = b'|';
                                junctions.insert(Junction { x: b, y: a });
                            }
                            _ => map[a][b] = b' ',
                        }
                    }
                }
                let track = Track { x, y, w, h };
                tracks.push(track);
            }
        }
    }
    loop {
        carts.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        for cart in carts {}
    }
}
