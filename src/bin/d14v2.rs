fn main() {
    let r = [9, 2, 0, 8, 3];
    let mut x = Vec::new();
    x.push(3);
    x.push(7);
    let mut e1 = 0;
    let mut e2 = 1;
    loop {
        let new = x[e1] + x[e2];
        if new > 9 {
            x.push(new / 10);
        }
        x.push(new % 10);
        e1 = (e1 + x[e1] + 1) % x.len();
        e2 = (e2 + x[e2] + 1) % x.len();
        if x.len() > 6 {
            if x[x.len() - 5..x.len()] == r {
                println!("{}", x.len() - 5);
                break;
            }
            if new > 9 {
                if x[x.len() - 6..x.len() - 1] == r {
                    println!("{}", x.len() - 6);
                    break;
                }
            }
        }
    }
}
