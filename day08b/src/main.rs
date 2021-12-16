struct SignalSeparator {
    one: u8,
    four: u8,
    seven: u8,
    len_five: Vec<u8>,
    len_six: Vec<u8>,
}

impl SignalSeparator {
    fn separate(signal: &Vec<&str>) -> Self {
        let mut one = 0;
        let mut four = 0;
        let mut seven = 0;
        let mut len_five: Vec<u8> = Vec::with_capacity(3);
        let mut len_six: Vec<u8> = Vec::with_capacity(3);

        for encoded_segment_string in signal {
            let len = encoded_segment_string.len();
            let encoded_display = segments_to_display(encoded_segment_string);

            match len {
                2 => { one = encoded_display; }
                3 => { seven = encoded_display; }
                4 => { four = encoded_display; }
                5 => { len_five.push(encoded_display); }
                6 => { len_six.push(encoded_display); }
                _ => {}
            }
        }

        Self { one, four, seven, len_five, len_six }
    }
}

struct WireConfig {
    zero: u8,
    one: u8,
    two: u8,
    three: u8,
    four: u8,
    five: u8,
    six: u8,
    seven: u8,
    eight: u8,
    nine: u8,
}

impl Default for WireConfig {
    fn default() -> Self {
        WireConfig {
            zero: 0b1110111,
            one: 0b0010010,
            two: 0b1011101,
            three: 0b1011011,
            four: 0b0111010,
            five: 0b1101011,
            six: 0b1101111,
            seven: 0b1010010,
            eight: 0b1111111,
            nine: 0b1111011,
        }
    }
}

impl WireConfig {
    fn from(separator: &SignalSeparator) -> Self {
        let mut config: WireConfig = Default::default();

        config.one = separator.one;
        config.four = separator.four;
        config.seven = separator.seven;

        for &display in separator.len_six.iter() {
            if display | config.one != display {
                config.six = display;
            } else if display | config.four == display {
                config.nine = display;
            } else {
                config.zero = display;
            }
        }

        for &display in separator.len_five.iter() {
            if display | config.one == display {
                config.three = display;
            } else if display | config.nine == config.nine {
                config.five = display;
            } else {
                config.two = display;
            }
        }

        config
    }
}

struct Entry<'a> {
    signal: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl Entry<'_> {
    fn decode_output(&self) -> i32 {
        let separator = SignalSeparator::separate(&self.signal);
        let config = WireConfig::from(&separator);

        self.output
            .iter()
            .map(|segment_string| segments_to_display(segment_string))
            .map(|display| display_to_digit(display, &config))
            .reduce(|number, digit| number * 10 + digit)
            .unwrap()
    }
}

fn segments_to_display(segment_string: &str) -> u8 {
    let mut display = 0u8;

    for c in segment_string.chars() {
        let segment = match c {
            'a' => 0b01,
            'b' => 0b010,
            'c' => 0b0100,
            'd' => 0b01000,
            'e' => 0b010000,
            'f' => 0b0100000,
            'g' => 0b01000000,
            _ => panic!("Error parsing char to segment"),
        };

        display |= segment;
    }

    display
}

const fn display_to_digit(display: u8, config: &WireConfig) -> i32 {
    match display {
        _ if display == config.zero => 0,
        _ if display == config.one => 1,
        _ if display == config.two => 2,
        _ if display == config.three => 3,
        _ if display == config.four => 4,
        _ if display == config.five => 5,
        _ if display == config.six => 6,
        _ if display == config.seven => 7,
        _ if display == config.eight => 8,
        _ if display == config.nine => 9,
        _ => unreachable!(),
    }
}

fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            let signal = split.next().unwrap().split(" ").collect::<Vec<_>>();
            let output = split.next().unwrap().split(" ").collect::<Vec<_>>();
            let entry = Entry { signal, output };
            entry.decode_output()
        })
        .sum::<i32>();

    println!("{:?}", result);
}
