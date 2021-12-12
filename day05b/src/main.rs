struct EitherWayRangeInclusive {
    start: usize,
    end: usize,
    cursor: usize,
    forward: bool,
    exhausted: bool,
}

impl EitherWayRangeInclusive {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end, cursor: start, forward: end > start, exhausted: false }
    }
}

impl Iterator for EitherWayRangeInclusive {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted { None }
        else if self.forward {
            if self.cursor >= self.end { 
                self.exhausted = true;
                None
            } else {
                let current = Some(self.cursor);

                if let Some(next) = self.cursor.checked_add(1) {
                    self.cursor = next;
                } else {
                    self.exhausted = true;
                    return None;
                }

                current
            }
        } else {
            if self.cursor <= self.end { 
                self.exhausted = true;
                None
            } else {
                let current = Some(self.cursor);

                if let Some(next) = self.cursor.checked_sub(1) {
                    self.cursor = next;
                } else {
                    self.exhausted = true;
                    return None;
                }

                current
            }
        }
    }
}

struct SegmentIterator {
    xs: EitherWayRangeInclusive,
    ys: EitherWayRangeInclusive
}

impl SegmentIterator {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        SegmentIterator {
            xs: EitherWayRangeInclusive::new(x1, x2),
            ys: EitherWayRangeInclusive::new(y1, y2)
        }
    }
}

impl Iterator for SegmentIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.xs.exhausted && self.ys.exhausted { None }
        else {
            let x = self.xs.next().or(Some(self.xs.end))?;
            let y = self.ys.next().or(Some(self.ys.end))?;
            Some((x, y))
        }
    }
}

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

            SegmentIterator::new(x1, y1, x2, y2)
        })
        .for_each(|segment| {
            for (x, y) in segment {
                field[y][x] += 1;
            }
        });

    let count = field
        .iter()
        .flatten()
        .filter(|n| **n > 1)
        .count();

    println!("{:?}", count);
}
