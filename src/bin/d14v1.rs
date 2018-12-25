fn print_state(x: &Vec<usize>, e1: usize, e2: usize) {
    for i in 0..x.len() {
        if i == e1 {
            if e1 == e2 {
                print!("<");
            } else {
                print!("(");
            }
        } else if i == e2 {
            print!("[");
        } else {
            print!(" ");
        }
        print!("{}", x[i]);
        if i == e1 {
            if e1 == e2 {
                print!(">");
            } else {
                print!(")");
            }
        } else if i == e2 {
            print!("]");
        } else {
            print!(" ");
        }
    }
    println!();
}

fn main() {
    let r = 920831;
    let mut x = Vec::new();
    x.push(3);
    x.push(7);
    let mut e1 = 0;
    let mut e2 = 1;
    while x.len() < r + 10 {
        //print_state(&x, e1, e2);
        let new = x[e1] + x[e2];
        if new > 9 {
            x.push(new / 10);
        }
        x.push(new % 10);
        e1 = (e1 + x[e1] + 1) % x.len();
        e2 = (e2 + x[e2] + 1) % x.len();
    }
    println!(
        "{}",
        x.iter()
            .skip(r)
            .take(10)
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
