fn main() {
    let mut scores = include_str!("../input.txt")
        .lines()
        .filter_map(|line| {
            let mut stack: Vec<char> = vec![];

            for symbol in line.chars() {
                if let Some(previous) = stack.pop() {
                    if symbol != pair_of(previous) {
                        if opens(symbol) {
                            stack.push(previous);
                            stack.push(symbol);
                        } else {
                            return None
                        }
                    }
                } else if opens(symbol) {
                    stack.push(symbol);
                }
            }

            if stack.len() > 0 {
                let mut score = 0;
                while let Some(symbol) = stack.pop() {
                    score = score * 5 + score_of(pair_of(symbol));
                }

                Some(score)
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    scores.sort();
    let middle = scores[scores.len() / 2];
    println!("{}", middle);
}

const fn score_of(symbol: char) -> u64 {
    match symbol {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!()
    }
}

const fn pair_of(symbol: char) -> char {
    match symbol {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!()
    }
}

const fn opens(symbol: char) -> bool {
    match symbol {
        '(' | '[' | '{' | '<' => true,
        _ => false
    }
}
