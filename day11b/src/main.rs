struct OctopusGrid {
    grid: Vec<Vec<u32>>
}

impl OctopusGrid {
    fn step(&mut self) -> u32 {
        self.grid
            .iter_mut()
            .flatten()
            .for_each(|n| *n += 1);

        let mut flash_map = [[false; 10]; 10];

        'outer: loop {
            for y in 0..10 {
                for x in 0..10 {
                    if self.grid[y][x] > 9 && !flash_map[y][x] {
                        self.flash_octopus_at(x, y);
                        flash_map[y][x] = true;
                        continue 'outer;
                    }
                }
            }

            break;
        }
    
        let mut number_of_flashes = 0;
        for y in 0..10 {
            for x in 0..10 {
                if flash_map[y][x] {
                    self.grid[y][x] = 0;
                    number_of_flashes += 1;
                }
            }
        }

        number_of_flashes
    }

    fn flash_octopus_at(&mut self, x: usize, y: usize) {
        let y_start = if y == 0 { 0 } else { y - 1 };
        let y_end = if y == 9 { 9 } else { y + 1 };
        let x_start = if x == 0 { 0 } else { x - 1 };
        let x_end = if x == 9 { 9 } else { x + 1 };

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                self.grid[y][x] += 1;
            }
        }
    }
}

fn main() {
    let grid = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect::<Vec<Vec<_>>>();

    let mut grid = OctopusGrid { grid };

    for i in 1.. {
        grid.step();

        let synchronized = !grid.grid
            .iter()
            .flatten()
            .any(|n| *n > 0);

        if synchronized {
            println!("{}", i);
            break;
        }
    }
}
