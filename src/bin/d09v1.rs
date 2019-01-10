fn print_board(f: &Vec<usize>, marble: usize, active: usize) {
    print!("[{:02}]  ", marble);
    for (index, x) in f.iter().enumerate() {
        print!("{:02}", x);
        if index == active {
            print!("(");
        } else if index == active + 1 {
            print!(")");
        } else {
            print!(" ");
        }
    }
    println!();
}

fn main() {
    let marbles: usize = 71035;
    let players: usize = 479;
    let mut score = vec![0usize; players];
    let mut f = Vec::with_capacity(marbles);
    f.push(0);
    let mut i = 0;
    println!("[--]  00");
    for marble in 1..marbles + 1 {
        if marble % 23 == 0 {
            let player = marble % players;
            score[player] += marble;
            i += f.len();
            i -= 7;
            i %= f.len();
            score[player] += f.remove(i + 1);
            print_board(&f, marble, i);
            continue;
        }
        i += 2;
        i %= f.len();
        f.insert(i + 1, marble);
        print_board(&f, marble, i);
    }
    println!("{}", score.iter().max().unwrap());
}
