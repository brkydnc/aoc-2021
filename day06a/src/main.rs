fn main() {
    let mut timers = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        let mut number_of_new_timers = 0;
        for timer in timers.iter_mut() {
            if *timer == 0 {
                number_of_new_timers += 1;
                *timer = 6;
            } else {
                *timer -= 1;
            }
        }

        for _ in 0..number_of_new_timers {
            timers.push(8);
        }
    }

    println!("{:?}", timers.len());
}
