use std::time::SystemTime;
use std::collections::VecDeque;

fn main() {
    let start = SystemTime::now();
    let marbles: usize = 7103500;
    let players: usize = 479;
    let mut score = vec![0usize; players];
    let mut f = VecDeque::with_capacity(marbles);
    f.push_back(0);
    let mut i = 0;
    for marble in 1..marbles + 1 {
        if marble % 23 == 0 {
            let player = marble % players;
            score[player] += marble;
            i += f.len();
            i -= 7;
            i %= f.len();
            let second = f.remove(i + 1);
            score[player] += second.unwrap();
        } else {
            i += 2;
            i %= f.len();
            f.insert(i + 1, marble);
        }
        if marble % 100000 == 0 {
            let since = SystemTime::now().duration_since(start).unwrap();
            println!("{}, {:?}", marble, since);
        }
    }
    println!("{}", score.iter().max().unwrap());
}
