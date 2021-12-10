fn main() {
    let gamma: u32 = include_str!("../input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .fold([0; 12], |mut total, mut n| {
            total[11] += n & 1;
            for i in (0..11).rev() {
                n >>= 1;
                total[i] += n & 1;
            }
            total
        })
        .into_iter()
        .fold(0, |gamma, n| {
            let bit = if n > 500 { 1 } else { 0 };
            (gamma << 1) + bit
        });

    let epsilon = !gamma & 4095;
    println!("{:?}", gamma * epsilon);
}
