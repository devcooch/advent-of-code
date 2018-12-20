fn fetch_children(mut iter: &mut std::slice::Iter<u32>) -> u32 {
    let mut total = 0u32;
    let child_count = iter.next().unwrap();
    let meta_count = iter.next().unwrap();
    for _ in 0..*child_count {
        total += fetch_children(&mut iter);
    }
    for _ in 0..*meta_count {
        total += fetch_meta(&mut iter);
    }
    total
}

fn fetch_meta(iter: &mut std::slice::Iter<u32>) -> u32 {
    *iter.next().unwrap()
}

fn main() {
    let data = include_str!("day08.txt");
    let numbers: Vec<u32> = data.trim().split(" ").map(|x| x.parse().unwrap()).collect();
    println!("{}", fetch_children(&mut numbers.iter()));
}
