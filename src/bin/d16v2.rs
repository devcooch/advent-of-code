use std::collections::HashSet;

fn get_in_brackets(s: &str) -> &str {
    let left = s.find('[').unwrap();
    let right = s.rfind(']').unwrap();
    &s[left + 1..right]
}

fn parse_nums(s: &str, split_by: &str) -> Vec<usize> {
    s.split(split_by)
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn addr(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] + regs[b];
    result
}

fn addi(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] + b;
    result
}

fn mulr(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] * regs[b];
    result
}

fn muli(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] * b;
    result
}

fn banr(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] & regs[b];
    result
}

fn bani(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] & b;
    result
}

fn borr(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] | regs[b];
    result
}

fn bori(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a] | b;
    result
}

fn setr(regs: &Vec<usize>, a: usize, _: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = regs[a];
    result
}

fn seti(regs: &Vec<usize>, a: usize, _: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = a;
    result
}

fn gtir(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = (a > regs[b]) as usize;
    result
}

fn gtri(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = (regs[a] > b) as usize;
    result
}

fn gtrr(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = (regs[a] > regs[b]) as usize;
    result
}

fn eqir(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = (a == regs[b]) as usize;
    result
}

fn eqri(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = (regs[a] == b) as usize;
    result
}

fn eqrr(regs: &Vec<usize>, a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut result = regs.clone();
    result[c] = (regs[a] == regs[b]) as usize;
    result
}

static INSTRUCTIONS: &'static [fn(&Vec<usize>, usize, usize, usize) -> Vec<usize>] = &[
    addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr,
];

fn guess(before: &Vec<usize>, asm: &Vec<usize>, after: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();
    for (index, instr) in INSTRUCTIONS.iter().enumerate() {
        if instr(before, asm[1], asm[2], asm[3]) == *after {
            result.push(index);
        }
    }
    result
}

fn execute(mapping: &Vec<usize>, asm: &Vec<usize>, regs: Vec<usize>) -> Vec<usize> {
    let opcode = asm[0];
    let a = asm[1];
    let b = asm[2];
    let c = asm[3];
    INSTRUCTIONS[mapping[opcode]](&regs, a, b, c)
}

fn find_mapping(to_guess: Vec<HashSet<usize>>) -> Vec<usize> {
    let mut v: Vec<_> = to_guess.iter().cloned().collect();
    let mut result = vec![None; to_guess.len()];
    while result.iter().any(|x| x.is_none()) {
        let index = v.iter().position(|x| x.len() == 1).unwrap();
        let value = v[index].iter().cloned().next().unwrap();
        result[index] = Some(value);
        for x in &mut v {
            x.remove(&value);
        }
    }
    result.iter().map(|x| x.unwrap()).collect()
}

fn main() {
    let data = include_str!("day16.txt");
    let mut iter = data.lines();
    let mut to_guess = vec![HashSet::new(); 16];
    loop {
        let first = iter.next().unwrap();
        if first.is_empty() {
            break;
        }
        let before = parse_nums(get_in_brackets(first), ", ");
        let asm = parse_nums(iter.next().unwrap(), " ");
        let after = parse_nums(get_in_brackets(iter.next().unwrap()), ", ");
        iter.next();

        let opcode = asm[0];
        let guessed: HashSet<usize> = guess(&before, &asm, &after).iter().cloned().collect();
        if to_guess[opcode].is_empty() {
            to_guess[opcode] = to_guess[opcode].union(&guessed).cloned().collect();
        } else {
            to_guess[opcode] = to_guess[opcode].intersection(&guessed).cloned().collect();
        }
    }
    let mapping = find_mapping(to_guess);
    let mut regs = vec![0usize; 4];
    loop {
        let possible_line = iter.next();
        if possible_line.is_none() {
            break;
        }
        let line = possible_line.unwrap();
        if line.is_empty() {
            continue;
        }
        let asm = parse_nums(line, " ");
        regs = execute(&mapping, &asm, regs);
    }
    println!("{}", regs[0]);
}
