#[derive(Debug)]
enum Axis {
    X,
    Y
}

const WIDTH: usize = 1311;
const HEIGHT: usize = 894;

fn main() {
    let mut paper = [[0; WIDTH]; HEIGHT];

    let mut lines = include_str!("../input.txt")
        .lines();

    let points = lines
        .by_ref()
        .take_while(|&line| line != "")
        .map(|line| {
            let mut split = line.split(',');
            let x = split.next().unwrap().parse::<usize>().unwrap();
            let y = split.next().unwrap().parse::<usize>().unwrap();

            (x, y)
        });

    for point in points {
        paper[point.1][point.0] = 1;
    }

    let instructions = lines
        .map(|line| {
            let mut split = line.split(' ').last().unwrap().split('=');
            let axis = match split.next().unwrap() {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => unreachable!()
            };
            let value = split.next().unwrap().parse::<usize>().unwrap();

            (axis, value)
        });

    let mut width = WIDTH;
    let mut height = HEIGHT;

    for instruction in instructions {
        match instruction {
            (Axis::X, fold) => {
                for y in 0..height {
                    for x in fold + 1..width {
                        paper[y][2 * fold - x] += paper[y][x];
                    }
                }

                width = fold;
            },
            (Axis::Y, fold) => {
                for y in fold + 1..height {
                    for x in 0..width {
                        paper[2 * fold - y][x] += paper[y][x];
                    }
                }

                height = fold;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if paper[y][x] > 0 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
