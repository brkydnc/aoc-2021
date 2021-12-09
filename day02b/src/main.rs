fn main() {
    let position = include_str!("../input.txt")
        .lines()
        .fold((0, 0, 0), |pos, line| {
            let mut rsplit = line.rsplit(' ');
            let value = rsplit.next().unwrap()
                .parse::<i32>().unwrap();

            match rsplit.next().unwrap() {
                "forward" => (pos.0 + value, pos.1 + pos.2 * value, pos.2),
                "up" => (pos.0, pos.1, pos.2 - value),
                "down" => (pos.0, pos.1, pos.2 + value),
                _ => unreachable!(),
            }
        });

    println!("{}", position.0 * position.1);
}
