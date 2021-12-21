use std::cmp::Ordering;

#[derive(Debug)]
struct Vertex {
    x: usize,
    y: usize,
    weight: usize,
    previous: Option<(usize, usize)>,
    distance: usize,
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
    let risk_levels = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut extended_rows = risk_levels.clone();
    
    for (i, row) in extended_rows.iter_mut().enumerate() {
        for j in 1..=4 {
            let mut row_clone = risk_levels[i].clone();
            
            row_clone
                .iter_mut()
                .for_each(|l| {
                    *l += j;
                    if *l > 9 {
                        *l = *l % 9;
                    }
                });
                
            row.append(&mut row_clone); 
        }
        
    }

    let mut extended_graph = extended_rows.clone();

    for i in 1..=4 {
        let mut extended_rows_clone = extended_rows.clone();
        
        for row in extended_rows_clone.iter_mut() {
            row
                .iter_mut()
                .for_each(|l| {
                    *l += i;
                    if *l > 9 {
                        *l = *l % 9;
                    }
                });
        }
        
        extended_graph.append(&mut extended_rows_clone);
    }

    let mut vertices = extended_graph
        .into_iter()
        .enumerate()
        .map(|(y, row)| row
            .into_iter()
            .enumerate()
            .map(move |(x, weight)| {
                Vertex {
                    x,
                    y,
                    weight: weight as usize,
                    previous: None,
                    distance: if x == 0 && y == 0 { 0 } else { usize::MAX }
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
                    (0, -1) | (-1, 0) | (0, 1) | (1, 0) => true,
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
