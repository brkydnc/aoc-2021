fn main() {
    let mut map = include_str!("../input.txt")
        .trim()
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map[0].len();

    let mut basin_sizes: Vec<u32> = vec![];
    for y in 0..height {
        for x in 0..width {
            let point = map[y][x];
            if point == 9 { continue; }

            let basin_size = get_basin_size(&mut map, x, y);
            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort();

    let mut product = 1;
    for _ in 0..3 {
        product *= basin_sizes.pop().unwrap();
    }

    println!("{}", product);
}

fn get_basin_size(map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let height = map.len();
    let width = map[0].len();

    let up = if y == 0 { y + 1 } else { y - 1 };
    let down = if y == height - 1 { y - 1 } else { y + 1 };
    let left = if x == 0 { x + 1 } else { x - 1 };
    let right = if x == width - 1 { x - 1 } else { x + 1 };

    let mut sum = 1;
    map[y][x] = 9;

    if map[up][x] != 9 {
        sum += get_basin_size(map, x, up);
    }
    
    if map[down][x] != 9  {
        sum += get_basin_size(map, x, down);
    }

    if map[y][left] != 9  {
        sum += get_basin_size(map, left, y);
    }

    if map[y][right] != 9  {
        sum += get_basin_size(map, right, y);
    }

    sum
}
