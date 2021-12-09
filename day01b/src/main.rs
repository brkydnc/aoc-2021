fn main() {
    let numbers = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap());

    let count = numbers
        .clone()
        .zip(numbers.skip(3))
        .fold(0, |total, (a, b)| if b > a { total + 1 } else { total });

        println!("{:?}", count);
}
