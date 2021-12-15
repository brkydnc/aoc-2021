fn main() {
    let mut positions = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let sum = positions
        .iter()
        .sum::<i32>();

    let mean = sum as f32 / positions.len() as f32;

    // https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
    let lower_bound = (mean - 0.5).floor() as i32;
    let upper_bound = (mean + 0.5).ceil() as i32;

    let mut min_cost = i32::MAX;
    for x in lower_bound..=upper_bound {
        let cost = positions
            .iter()
            .fold(0, |sum, n| {
                let distance = (x - n).abs();
                sum + distance * (distance + 1) / 2
            });

        if min_cost > cost { min_cost = cost; };
    }

    println!("{:?}", min_cost);
}
