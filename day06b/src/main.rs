fn main() {
    let mut timers = [0u64; 9];

    include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| timers[n] += 1);

    for _ in 0..256 {
        let zeroes = timers[0];
        timers.rotate_left(1);
        timers[6] += zeroes;
    }

     let population = timers
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{:?}", population);
}
