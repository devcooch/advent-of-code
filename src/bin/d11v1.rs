fn main() {
    let sn = 4151;
    let mut cell = vec![0_i32; 300 * 300];
    for i in 1..301 {
        for j in 1..301 {
            let rack_id = (j as i32) + 10;
            let mut power: i32 = rack_id * (i as i32);
            power += sn;
            power *= rack_id;
            power /= 100;
            power %= 10;
            power -= 5;
            cell[(i - 1 + (j - 1) * 300) as usize] = power;
        }
    }
    let mut max_pos = (0, 0);
    let mut max_energy = 0;
    for i in 0_usize..298_usize {
        for j in 0_usize..298_usize {
            let mut energy: i32 = cell[i + j * 300..i + 3 + j * 300].iter().sum();
            energy += cell[i + (j + 1) * 300..i + 3 + (j + 1) * 300]
                .iter()
                .sum::<i32>();
            energy += cell[i + (j + 2) * 300..i + 3 + (j + 2) * 300]
                .iter()
                .sum::<i32>();
            if energy > max_energy {
                max_energy = energy;
                max_pos = (i, j);
            }
        }
    }
    println!("{},{}", max_pos.1 + 1, max_pos.0 + 1);
    println!("{}", max_energy);
}
