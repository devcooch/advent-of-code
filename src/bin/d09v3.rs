use std::collections::BTreeMap;

fn main() {
    let marbles: usize = 5807;
    let players: usize = 30;
    let mut score = vec![0usize; players];
    let mut f = BTreeMap::new();
    f.insert(0usize, 0usize);
    let mut i = 0;
    for marble in 1..marbles + 1 {
        if marble % 23 == 0 {
            let player = marble % players;
            score[player] += marble;
            i += f.len();
            i -= 7;
            i %= f.len();
            let key = *f.iter().find(|(k, v)| **v == i + 1).unwrap().0;
            score[player] += key;
            f.remove(&key);
            for v in f.values_mut() {
                if v > &mut (i + 1) {
                    *v -= 1;
                }
            }
        } else {
            i += 2;
            i %= f.len();
            f.insert(marble, i + 1);
            for v in f.values_mut() {
                if v > &mut i {
                    *v += 1;
                }
            }
        }
        if marble % 100 == 0 {
            println!("{}", marble);
        }
    }
    println!("{}", score.iter().max().unwrap());
}
