fn main() {
    let mut positions = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    positions.sort();

    let cheapest = positions[(positions.len() - 1) / 2];

    let cost = positions
        .into_iter()
        .map(|n| (cheapest - n).abs())
        .sum::<i32>();

    println!("{:?}", cost);
}
