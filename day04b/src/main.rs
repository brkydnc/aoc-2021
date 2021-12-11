enum MarkResult {
    Win(i32),
    Mark,
    None,
}

struct Cell {
    number: i32,
    marked: bool,
}

struct Board {
    rows: Vec<Vec<Cell>>,
    has_ended: bool,
}

impl Board {
    fn mark(&mut self, number: i32) -> MarkResult {
        let mut marked_position = None;

        for (y, row) in self.rows.iter_mut().enumerate() {
            if let Some(x) = row.iter().position(|cell| cell.number == number) {
                row[x].marked = true;
                marked_position = Some((x, y));
                break;
            }
        }

        if let Some((x, y)) = marked_position {
            let mut row_marked = true;
            for x in 0..self.rows[y].len() {
                if !self.rows[y][x].marked {
                    row_marked = false;
                    break;
                }
            }

            let mut column_marked = true;
            for y in 0..self.rows.len() {
                if !self.rows[y][x].marked {
                    column_marked = false;
                    break;
                }
            }

            if row_marked || column_marked {
                return MarkResult::Win(self.sum_of_unmarked() * number);
            }

            MarkResult::Mark
        } else {
            MarkResult::None
        }
    }

    fn sum_of_unmarked(&self) -> i32 {
        self.rows
            .iter()
            .fold(0, |acc, row| {
                let row_sum = row
                    .iter()
                    .fold(0, |acc, cell| {
                        if !cell.marked {
                            acc + cell.number
                        } else {
                            acc
                        }
                    });

                acc + row_sum
            })
    }

    fn end(&mut self) {
        self.has_ended = true;
    }
}

fn main() {
    let mut lines = include_str!("../input.txt")
        .lines()
        .filter(|line| !line.is_empty());

    let bingos = lines.next().unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap());

    let mut boards: Vec<Board> = Vec::with_capacity(100);
    for _ in 0..100 {
        let rows = lines
            .by_ref()
            .take(5)
            .map(|row| {
                row
                    .split(' ')
                    .filter(|line| !line.is_empty())
                    .map(|n| {
                        let number = n.parse::<i32>().unwrap();
                        Cell { number, marked: false }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();

        boards.push(Board { rows, has_ended: false });
    }

    let mut last_score = 0;
    for bingo in bingos {
        for board in boards.iter_mut() {
            if board.has_ended { continue; }

            match board.mark(bingo) {
                MarkResult::Mark | MarkResult::None => {},
                MarkResult::Win(score) => {
                    last_score = score;
                    board.end();
                }
            }
        }
    }

    println!("{}", last_score);
}
