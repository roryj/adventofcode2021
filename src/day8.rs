use std::fmt::Display;

pub fn part1(input: Vec<String>) -> u32 {
    let wire_outputs: Vec<Option<u32>> = input
        .clone()
        .iter()
        .map(|l| {
            let split: Vec<&str> = l.split("|").collect();
            let _inputs = split.get(0).unwrap().trim();
            let outputs = split.get(1).unwrap().trim();

            let o: Vec<Option<u32>> = outputs
                .split_whitespace()
                .map(|o| try_get_output_digit_part_1(o))
                .collect();

            o
        })
        .flatten()
        .collect();

    wire_outputs
        .iter()
        .map(|s| match s {
            Some(_) => 1,
            None => 0,
        })
        .sum()
}

fn get_output_digit_part_2(num: String, one_chars: &str, four_chars: &str) -> u32 {
    match try_get_output_digit_part_1(num.clone()) {
        Some(v) => v,
        None => {
            match (
                num.len(),
                num.chars().filter(|c| one_chars.contains(*c)).count(),
                num.chars().filter(|c| four_chars.contains(*c)).count(),
            ) {
                (5, 1, 3) => 5,
                (5, 2, 3) => 3,
                (5, _, 2) => 2,
                (6, 1, _) => 6,
                (6, _, 3) => 0,
                (6, _, 4) => 9,
                _ => unreachable!(),
            }
        }
    }
}

fn try_get_output_digit_part_1<P: ToString>(num: P) -> Option<u32> {
    match num.to_string().len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

pub fn part2(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|l| {
            let split: Vec<&str> = l.split("|").collect();
            let inputs: Vec<&str> = split.get(0).unwrap().trim().split_whitespace().collect();
            let outputs = split.get(1).unwrap().trim();
            let one = inputs.iter().find(|d| d.len() == 2).unwrap();
            let four = inputs.iter().find(|d| d.len() == 4).unwrap();

            let o: Vec<u32> = outputs
                .split_whitespace()
                .map(|s| get_output_digit_part_2(s.to_string(), one, four))
                .collect();

            o.iter()
                .enumerate()
                .fold(0, |acc, (i, n)| acc + n * 10_u32.pow(3 - i as u32))
            // .map(|(i, v)| {
            //     let pow: u32 = (o.len() - i - 1) as u32;

            //     v * (10 as u32).pow(pow)
            // })
            // .sum::<u32>()
        })
        .sum()
}

#[derive(Debug, Clone)]
struct SevenSegment {
    segments: [bool; 7],
}

impl SevenSegment {
    const one_bitmap: [bool; 7] = [true, true, false, false, false, false, false];
    const two_bitmap: [bool; 7] = [true, false, true, true, false, true, true];
    const three_bitmap: [bool; 7] = [true, true, true, true, false, true, false];
    const four_bitmap: [bool; 7] = [true, true, false, false, true, true, false];
    const five_bitmap: [bool; 7] = [false, true, true, true, true, true, false];
    const six_bitmap: [bool; 7] = [false, true, true, true, true, true, true];
    const seven_bitmap: [bool; 7] = [true, true, false, true, false, false, false];
    const eight_bitmap: [bool; 7] = [true, true, true, true, true, true, true];

    pub fn new<P: ToString>(input: P) -> Self {
        let mut segments = [false; 7];

        for c in input.to_string().chars() {
            match c {
                'a' => segments[0] = true,
                'b' => segments[1] = true,
                'c' => segments[2] = true,
                'd' => segments[3] = true,
                'e' => segments[4] = true,
                'f' => segments[5] = true,
                'g' => segments[6] = true,
                v => panic!("unexpected char: {}", v),
            }
        }

        Self { segments }
    }

    pub fn one() -> Self {
        SevenSegment::new("cf")
    }
    pub fn four() -> Self {
        SevenSegment::new("bcdf")
    }

    pub fn seven() -> Self {
        SevenSegment::new("acf")
    }
    pub fn eight() -> Self {
        SevenSegment::new("abcdefg")
    }
    pub fn get_digit(&self) -> u32 {
        match self.try_get_digit() {
            Some(d) => d,
            None => panic!("Expected a value here:\n {}", self),
        }
    }

    pub fn try_get_digit(&self) -> Option<u32> {
        match self.segments {
            SevenSegment::one_bitmap => Some(1),
            SevenSegment::two_bitmap => Some(2),
            SevenSegment::three_bitmap => Some(3),
            SevenSegment::four_bitmap => Some(4),
            SevenSegment::five_bitmap => Some(5),
            SevenSegment::six_bitmap => Some(6),
            SevenSegment::seven_bitmap => Some(7),
            SevenSegment::eight_bitmap => Some(8),
            _ => None,
        }
    }
}

impl Display for SevenSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self.segments[0] {
            true => "a",
            false => ".",
        };
        let b = match self.segments[1] {
            true => "b",
            false => ".",
        };
        let c = match self.segments[2] {
            true => "c",
            false => ".",
        };
        let d = match self.segments[3] {
            true => "d",
            false => ".",
        };
        let e = match self.segments[4] {
            true => "e",
            false => ".",
        };
        let f_str = match self.segments[5] {
            true => "f",
            false => ".",
        };
        let g = match self.segments[6] {
            true => "g",
            false => ".",
        };

        f.write_fmt(format_args!(" {}{}{}{} \n", d, d, d, d))
            .unwrap();
        f.write_fmt(format_args!("{}    {}\n", e, a)).unwrap();
        f.write_fmt(format_args!("{}    {}\n", e, a)).unwrap();
        f.write_fmt(format_args!(" {}{}{}{} \n", f_str, f_str, f_str, f_str))
            .unwrap();
        f.write_fmt(format_args!("{}    {}\n", g, b)).unwrap();
        f.write_fmt(format_args!("{}    {}\n", g, b)).unwrap();
        f.write_fmt(format_args!(" {}{}{}{} \n", c, c, c, c))
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, SevenSegment};

    #[test]
    fn test_seven_segment() {
        let one = SevenSegment::one();
        let four = SevenSegment::four();
        let seven = SevenSegment::seven();
        let eight = SevenSegment::eight();

        println!("{}", one);
        println!("{}", four);
        println!("{}", seven);
        println!("{}", eight);
    }

    #[test]
    fn test_part1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
                .lines()
                .map(|s| s.to_string())
                .collect();

        let result = part1(input);
        assert_eq!(26, result);
    }

    #[test]
    fn test_part2() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
                .lines()
                .map(|s| s.to_string())
                .collect();

        let result = part2(input);
        assert_eq!(61229, result);
    }
}
