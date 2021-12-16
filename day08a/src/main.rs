fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|line| line.split(" | ").skip(1).next().unwrap().split(' '))
        .map(|digits| {
            let mut sum = 0;
            for digit in digits {
                match digit.len() {
                    2 | 3 | 4 | 7 => { sum += 1; },
                    _ => {}
                }
            }
            sum
        })
        .sum::<i32>();

        println!("{}", result);
}
