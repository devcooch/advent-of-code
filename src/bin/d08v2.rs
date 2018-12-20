fn fetch_children(mut iter: &mut std::slice::Iter<u32>) -> u32 {
    let mut total = 0u32;
    let child_count = *iter.next().unwrap();
    let meta_count = *iter.next().unwrap();
    if child_count == 0 {
        for _ in 0..meta_count {
            total += fetch_meta(&mut iter);
        }
    } else {
        let mut values = Vec::with_capacity(child_count as usize);
        for _ in 0..child_count {
            values.push(fetch_children(&mut iter));
        }
        for _ in 0..meta_count {
            let meta = fetch_meta(&mut iter);
            if meta == 0 || meta > child_count {
                continue;
            }
            total += values[meta as usize - 1];
        }
    }
    total
}

fn fetch_meta(iter: &mut std::slice::Iter<u32>) -> u32 {
    *iter.next().unwrap()
}

fn main() {
    let data = include_str!("day08.txt");
    let numbers: Vec<u32> = data.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    println!("{}", fetch_children(&mut numbers.iter()));
}
