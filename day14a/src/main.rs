#![feature(slice_group_by)]
use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../input.txt")
        .lines();

    let template = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<_>>();

    let rules = lines.skip(1);
        
    let mut quantities = rules
        .clone()
        .map(|rule| {
            let mut split = rule.split(" -> ");
            let mut pair_split = split.next().unwrap().chars();

            let pair_first = pair_split.next().unwrap();
            let pair_last = pair_split.next().unwrap();

            let pair = (pair_first, pair_last);
            (pair, 0i64)
        })
        .collect::<HashMap<_, _>>();

    let mut pairs_to_insert = quantities.clone();

    let products = rules
        .map(|rule| {
            let mut split = rule.split(" -> ");
            let mut pair_split = split.next().unwrap().chars();

            let pair_first = pair_split.next().unwrap();
            let pair_last = pair_split.next().unwrap();

            let element = split.next().unwrap().chars().next().unwrap();

            let pair = (pair_first, pair_last);
            (pair, ((pair_first, element), (element, pair_last)))
        })
        .collect::<HashMap<_, _>>();

    for window in template.windows(2) {
        let pair_left = window[0];
        let pair_right = window[1];
        let pair = (pair_left, pair_right);

        let quantity = quantities
            .get_mut(&pair)
            .unwrap();

        *quantity += 1;
    }

    for _ in 0..10 {
        for (pair, quantity) in &quantities {
            let (product1, product2) = products.get(&pair).unwrap();
            *pairs_to_insert.get_mut(&pair).unwrap() -= quantity;
            *pairs_to_insert.get_mut(product1).unwrap() += quantity;
            *pairs_to_insert.get_mut(product2).unwrap() += quantity;
        }

        for (pair, quantity) in pairs_to_insert.iter_mut() {
            *quantities.get_mut(&pair).unwrap() += *quantity;
            *quantity = 0;
        }
    }

    let mut elements: HashMap<char, i64> = HashMap::new();

    for ((lhs, rhs), quantity) in quantities {
        if quantity == 0 { continue; };

        elements
            .entry(lhs)
            .and_modify(|n| *n += quantity)
            .or_insert(quantity);

        elements
            .entry(rhs)
            .and_modify(|n| *n += quantity)
            .or_insert(quantity);
    }

    elements.iter_mut().for_each(|(_, quantity)| *quantity /= 2);

    elements.entry(template[0]).and_modify(|n| *n += 1);
    elements.entry(template[template.len() - 1]).and_modify(|n| *n += 1);

    let (_, max) = elements
        .iter()
        .max_by(|(_, q1), (_, q2)| q1.cmp(q2))
        .unwrap();

    let (_, min) = elements
        .iter()
        .min_by(|(_, q1), (_, q2)| q1.cmp(q2))
        .unwrap();

    println!("{:?}", max - min);
}
