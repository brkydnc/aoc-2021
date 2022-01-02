use std::collections::HashMap;

struct Cave<'a> {
    small: bool,
    adjacents: Vec<&'a str>
}

impl<'a> Cave<'a> {
    fn new(label: &str, adjacents: Vec<&'a str>) -> Self {
        Self {
            small: label == label.to_lowercase(),
            adjacents
        }
    }
}

struct Map<'a> {
    caves: HashMap<&'a str, Cave<'a>>
}

impl<'a> Map<'a> {
    fn new() -> Self {
        Self { caves: HashMap::new() }
    }

    fn number_of_distinct_paths(&self, label: &'a str, mut path: Vec<&'a str>, mut visited_single_cave_twice: bool) -> i32 {
        if label == "end" { return 1; }

        let source = self.caves.get(label).unwrap();

        if source.small && path.contains(&label) { visited_single_cave_twice = true }
        path.push(label);

        source.adjacents
            .iter()
            .filter(|label| {
                let cave = self.caves.get(*label).unwrap();
                let visitable = !cave.small || !path.contains(label) || !visited_single_cave_twice;
                let exception = **label == "start";

                visitable && !exception
            })
            .map(|label| self.number_of_distinct_paths(label, path.clone(), visited_single_cave_twice))
            .sum()
    }
}

fn main() {
    let mut map = Map::new();

    include_str!("../input.txt")
        .lines()
        .for_each(|line| {
            let (a, b) = line.split_once("-").unwrap();

            map.caves
                .entry(a)
                .and_modify(|cave| cave.adjacents.push(b))
                .or_insert(Cave::new(a, vec![b]));

            map.caves
                .entry(b)
                .and_modify(|cave| cave.adjacents.push(a))
                .or_insert(Cave::new(b, vec![a]));
        });

    let result = map.number_of_distinct_paths("start", vec![], false);
    println!("{:?}", result);
}
