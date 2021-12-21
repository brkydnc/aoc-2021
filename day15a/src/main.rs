use std::cmp::Ordering;

#[derive(Debug)]
struct Vertex {
    x: usize,
    y: usize,
    weight: u32,
    previous: Option<(usize, usize)>,
    distance: u32,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Vertex { }

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn main() {
    let lines = include_str!("../input.txt")
        .lines();

    let mut vertices = lines
        .clone()
        .enumerate()
        .map(|(y, line)| line
             .chars()
             .map(|c| c.to_digit(10).unwrap())
             .enumerate()
             .map(move |(x, weight)| {
                 Vertex {
                     x,
                     y,
                     weight,
                     previous: None,
                     distance: if x == 0 && y == 0 { 0 } else { u32::MAX }
                 }
             }))
        .flatten()
        .collect::<Vec<_>>();

    vertices[0].distance = 0;
    vertices.reverse();

    let mut lowest_total_risk = 0;
    let target = (vertices[0].x, vertices[0].y);

    while let Some(vertex) = vertices.pop() {
        if vertex.x == target.0 && vertex.y == target.1 {
            lowest_total_risk = vertex.distance;
        }

        let neighbours = vertices
            .iter_mut()
            .filter(|neighbour| {
                let x_diff = vertex.x as isize - neighbour.x as isize;
                let y_diff = vertex.y as isize - neighbour.y as isize;

                match (x_diff, y_diff) {
                    (0, -1..=1) | (-1..=1, 0) => true,
                    _ => false
                }
            });

        for neighbour in neighbours {
            let cost = vertex.distance + neighbour.weight;
            if neighbour.previous.is_some() {
                if cost < neighbour.distance {
                    neighbour.previous.replace((vertex.x, vertex.y));
                    neighbour.distance = cost;
                }
            } else {
                neighbour.previous.replace((vertex.x, vertex.y));
                neighbour.distance = cost;
            }
        }

        vertices.sort_by(|a, b| b.cmp(a));
    }

    println!("{:?}", lowest_total_risk);
}
