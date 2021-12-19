fn main() {
    let mut pair_quantities = [[0isize; 26]; 26];
    let mut pair_products = [[0; 26]; 26];

    let mut lines = include_str!("../input.txt")
        .lines()
        .filter(|line| !line.is_empty());

    let template = lines.next().unwrap()
        .chars()
        .map(alphabet_index)
        .collect::<Vec<_>>();

    for window in template.windows(2) {
        pair_quantities[window[1]][window[0]] += 1;
    }

    for rule in lines {
        let mut split = rule.split(" -> ");
        let mut pair = split.next().unwrap().chars();
        let lhs = alphabet_index(pair.next().unwrap());
        let rhs = alphabet_index(pair.next().unwrap());
        let product = alphabet_index(split.next().unwrap().chars().next().unwrap());

        pair_products[rhs][lhs] = product;
    }

    let mut quantities_to_add = [[0; 26]; 26];

    for _ in 0..40 {
        for lhs in 0..26 {
            for rhs in 0..26 {
                let quantity = pair_quantities[rhs][lhs];
                if quantity == 0 { continue; }

                let product = pair_products[rhs][lhs];
                quantities_to_add[rhs][lhs] -= quantity;
                quantities_to_add[product][lhs] += quantity;
                quantities_to_add[rhs][product] += quantity;
            }
        }

        for lhs in 0..26 {
            for rhs in 0..26 {
                let quantity = quantities_to_add[rhs][lhs];
                if quantity == 0 { continue; }

                pair_quantities[rhs][lhs] += quantity;
                quantities_to_add[rhs][lhs] = 0;
            }
        }
    }

    let mut element_quantities = [0; 26];

    for lhs in 0..26 {
        for rhs in 0..26 {
            let quantity = pair_quantities[rhs][lhs];
            if quantity == 0 { continue; }

            element_quantities[lhs] += quantity;
            element_quantities[rhs] += quantity;
        }
    }

    element_quantities.iter_mut().for_each(|q| *q /= 2);

    element_quantities[template[0]] += 1;
    element_quantities[template[template.len() - 1]] += 1;

    let max = element_quantities
        .iter()
        .filter(|n| **n != 0)
        .max()
        .unwrap();

    let min = element_quantities
        .iter()
        .filter(|n| **n != 0)
        .min()
        .unwrap();

    println!("{:?}", max - min);
}

fn alphabet_index(c: char) -> usize {
    (c as u8 - b'A') as usize
}
