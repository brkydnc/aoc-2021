#![feature(int_roundings)]

fn main() {
    let input_numbers = include_str!("../input.txt")
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u32>>();

    let o2 = get_rating(input_numbers.clone(), true);
    let co2 = get_rating(input_numbers, false);

    println!("{:?}", o2 * co2);
}

fn get_rating(mut numbers: Vec<u32>, oxygen: bool) -> u32 {
    for position in (0..12).rev() {
        let number_of_ones = numbers
            .iter()
            .fold(0, |count, n| {
                if (n >> position) & 1 == 1 {
                    count + 1
                } else {
                    count
                }
            });

        let len = numbers.len();
        if len == 1 { break; };

        let filter_bit = if oxygen {
            if number_of_ones >= len.unstable_div_ceil(2) { 1 } else { 0 }
        } else {
            if number_of_ones < len.unstable_div_ceil(2) { 1 } else { 0 }
        };

        numbers = numbers
            .into_iter()
            .filter(|&n| (n >> position) & 1 == filter_bit)
            .collect();
    }

    numbers[0]
}
