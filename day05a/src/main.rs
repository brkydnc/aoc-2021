fn main() {
    let mut field = [[0; 990]; 990];

    let mut segments = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let start = split.next().unwrap();
            let end = split.next().unwrap();
            (start, end)
        })
        .map(|(start, end)| {
            let mut start_split = start.split(',');
            let mut end_split = end.split(',');

            let x1 = start_split.next().unwrap().parse::<usize>().unwrap();
            let y1 = start_split.next().unwrap().parse::<usize>().unwrap();
            let x2 = end_split.next().unwrap().parse::<usize>().unwrap();
            let y2 = end_split.next().unwrap().parse::<usize>().unwrap();

            (x1, y1, x2, y2)
        });

    for (x1, y1, x2, y2) in segments {
        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                field[y][x1] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                let i = y1 * 1000 + x;
                field[y1][x] += 1;
            }
        }
    }

    let count = field
        .iter()
        .flatten()
        .filter(|n| **n > 1)
        .count();

    println!("{:?}", count);
}
