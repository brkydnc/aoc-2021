fn main() {
    let file_score = include_str!("../input.txt")
        .lines()
        .fold(0, |total_score, line| {
            let mut stack: Vec<char> = vec![];

            for symbol in line.chars() {
                if let Some(previous) = stack.pop() {
                    if symbol != pair_of(previous) {
                        if opens(symbol) {
                            stack.push(previous);
                            stack.push(symbol);
                        } else {
                            return total_score + score_of(symbol);
                        }
                    }
                } else if opens(symbol) {
                    stack.push(symbol);
                } else {
                    panic!("Symbol closes empty stack");
                }
            }

            total_score
        });

    println!("{}", file_score);
}

const fn score_of(symbol: char) -> i32 {
    match symbol {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
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
