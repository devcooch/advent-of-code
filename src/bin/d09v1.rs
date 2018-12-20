fn main() {
    let marbles = 71035;
    let players = 479;
    let mut score = vec![0; players];
    let mut f = Vec::with_capacity(marbles);
    f.push(0);
    let mut i = 0;
    for marble in 1..marbles {
        if marble % 23 == 0 {
            let player = marble % players;
            score[player] += marble;
            i += f.len();
            i -= 7;
            i %= f.len();
            score[player] += f.remove(i);
            continue;
        }
        i += 2;
        i %= f.len();
        f.insert(i, marble);
    }
    println!("{}", score.iter().max().unwrap());
}
