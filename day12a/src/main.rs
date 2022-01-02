use std::collections::HashMap;

struct Cave<'a> {
    visitable_once: bool,
    adjacents: Vec<&'a str>
}

impl<'a> Cave<'a> {
    fn new(label: &str, adjacents: Vec<&'a str>) -> Self {
        Self {
            visitable_once: label == label.to_lowercase(),
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

    fn number_of_distinct_paths(&self, source_label: &'a str, mut path: Vec<&'a str>) -> i32 {
        if source_label == "end" { return 1; }

        let source = self.caves.get(source_label).unwrap();
        path.push(source_label);

        source.adjacents
            .iter()
            .filter(|cave| !self.caves.get(*cave).unwrap().visitable_once || !path.contains(cave))
            .map(|cave| self.number_of_distinct_paths(cave, path.clone()))
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

    let result = map.number_of_distinct_paths("start", vec![]);
    println!("{:?}", result);
}
