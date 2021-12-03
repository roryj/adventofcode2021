pub fn part1(input: Vec<String>) -> u32 {
    let mut byte_list = vec![
        ByteValue {
            num_ones: 0,
            num_zeros: 0
        };
        input[0].len()
    ];
    input.iter().for_each(|bytes| {
        for (index, c) in bytes.chars().enumerate() {
            let mut value = byte_list[index];
            value.saw(c);
            byte_list[index] = value;
        }
    });

    let gamma_rate_bits = byte_list
        .iter()
        .map(|b| b.get_most_common())
        .collect::<String>();
    let epsilon_rate_bits = byte_list
        .iter()
        .map(|b| b.get_least_common())
        .collect::<String>();
    // println!("gamma {}, epsilon {}", gamma_rate_bits, epsilon_rate_bits);

    let gamma_rate = convert_to_decimal(gamma_rate_bits);
    let epsilon_rate = convert_to_decimal(epsilon_rate_bits);

    // println!("INPUT: {:?}", input);
    // println!("Result: {:?}", byte_list);
    // println!("gamma {}, epsilon {}", gamma_rate, epsilon_rate);

    gamma_rate * epsilon_rate
}

fn convert_to_decimal(bit_str: String) -> u32 {
    let mut result = 0;
    let base: u32 = 2;

    for (index, c) in bit_str.chars().enumerate() {
        let x = c.to_string().parse::<u32>().unwrap();

        result += x * base.pow(bit_str.len() as u32 - index as u32 - 1)
    }

    result
}

#[derive(Debug, Clone, Copy)]
struct ByteValue {
    num_zeros: u32,
    num_ones: u32,
}

impl ByteValue {
    fn saw(&mut self, c: char) {
        match c {
            '0' => self.num_zeros += 1,
            '1' => self.num_ones += 1,
            v => panic!("unexpected char: {}", v),
        }
    }

    fn get_most_common(self) -> String {
        if self.num_zeros > self.num_ones {
            return "0".to_string();
        }

        "1".to_string()
    }

    fn get_least_common(self) -> String {
        if self.num_zeros > self.num_ones {
            return "1".to_string();
        }

        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_part1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .lines()
            .map(|s| String::from(s))
            .collect();

        let result = part1(input);

        assert_eq!(result, 198)
    }
}
