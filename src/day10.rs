pub fn part1(input: Vec<String>) -> u32 {
    let result = input
        .iter()
        .map(|row| {
            let mut stack: Vec<char> = Vec::new();

            for c in row.chars() {
                if vec!['<', '{', '(', '['].contains(&c) {
                    stack.push(c);
                    continue;
                }

                let error_result: Option<u32> = match c {
                    '>' => match stack.pop() {
                        Some(c) if c == '<' => None,
                        _ => Some(25137),
                    },
                    '}' => match stack.pop() {
                        Some(c) if c == '{' => None,
                        _ => Some(1197),
                    },
                    ')' => match stack.pop() {
                        Some(c) if c == '(' => None,
                        _ => Some(3),
                    },
                    ']' => match stack.pop() {
                        Some(c) if c == '[' => None,
                        _ => Some(57),
                    },
                    _ => unreachable!(),
                };

                if let Some(r) = error_result {
                    return r;
                }
            }
            0
        })
        .sum();

    result
}

pub fn part2(input: Vec<String>) -> usize {
    // println!("starting: {:?}", input);
    let mut result = input
        .iter()
        .map(|row| {
            // println!("the row: {}", row);

            let mut stack: Vec<char> = Vec::new();

            for c in row.chars() {
                if vec!['<', '{', '(', '['].contains(&c) {
                    stack.push(c);
                    continue;
                }

                match c {
                    '>' => match stack.pop() {
                        Some(c) if c == '<' => continue,
                        _ => return 0,
                    },
                    '}' => match stack.pop() {
                        Some(c) if c == '{' => continue,
                        _ => return 0,
                    },
                    ')' => match stack.pop() {
                        Some(c) if c == '(' => continue,
                        _ => return 0,
                    },
                    ']' => match stack.pop() {
                        Some(c) if c == '[' => continue,
                        _ => return 0,
                    },
                    _ => unreachable!(),
                }
            }

            // println!("after process: {:?} -> {:?}", row, stack);

            let result = stack.iter().rev().fold(0, |acc, n| {
                let v = match n {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };

                acc * 5 + v
            });

            // println!("found sum for {} => {:?}", row, result);

            result
        })
        .filter(|a| *a > 0)
        .collect::<Vec<usize>>();

    result.sort();

    // println!("result: {:?}", result);

    let mid_point = (result.len() / 2) as usize;

    result[mid_point]
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            .lines()
            .map(|l| l.to_string())
            .collect();

        let result = part1(input);
        assert_eq!(26397, result);
    }

    #[test]
    fn test_part2() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            .lines()
            .map(|l| l.to_string())
            .collect();

        let result = part2(input);
        println!("{:?}", result);
        assert!(false);
    }
}
