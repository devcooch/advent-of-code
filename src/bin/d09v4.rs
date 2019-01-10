use std::time::SystemTime;

fn i_to_pos_k(i: usize, f: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut pos = i + 1;
    let mut k = 0;
    while pos > f[k].len() {
        pos -= f[k].len();
        k += 1;
    }
    (pos, k)
}

fn main() {
    let start = SystemTime::now();
    let marbles: usize = 7103500;
    let players: usize = 479;
    let mut score = vec![0usize; players];
    let mut l: usize = 1;
    let mut f = Vec::new();
    f.push(vec![0]);
    let mut i = 0;
    for marble in 1..marbles + 1 {
        if marble % 23 == 0 {
            let player = marble % players;
            score[player] += marble;
            i += l;
            i -= 7;
            i %= l;
            let (pos, k) = i_to_pos_k(i, &f);
            if pos < f[k].len() {
                let tail = f[k].split_off(pos + 1);
                score[player] += f[k].pop().unwrap();
                l -= 1;
                f.insert(k + 1, tail);
            } else {
                score[player] += f[k + 1].remove(0);
                l -= 1;
            }
        } else {
            i += 2;
            i %= l;
            let (pos, k) = i_to_pos_k(i, &f);
            f[k].insert(pos, marble);
            l += 1
        }
        if marble % 100000 == 0 {
            let since = SystemTime::now().duration_since(start).unwrap();
            println!("{}, {:?}", marble, since);
        }
    }
    println!("{}", score.iter().max().unwrap());
}
