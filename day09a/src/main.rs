fn main() {
    let rows = include_str!("../input.txt")
        .trim()
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = rows.len();
    let width = rows[0].len();

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let up = if y == 0 { rows[y + 1][x] } else { rows[y - 1][x] };
            let down = if y == height - 1 { rows[y - 1][x] } else { rows[y + 1][x] };
            let left = if x == 0 { rows[y][x + 1] } else { rows[y][x - 1] };
            let right = if x == width - 1 { rows[y][x - 1] } else { rows[y][x + 1] };

            let point = rows[y][x];

            if up > point && down > point && left > point && right > point {
                sum += point + 1;
            }
        }
    }

    println!("{:?}", sum);
}
