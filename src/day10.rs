enum Delimeter {
    ParenOpen,
    ParenClose,

    SquareOpen,
    SquareClose,

    BracketOpen,
}

pub fn part1(input: Vec<String>) -> u32 {
    // stack
    //

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

                // match c {
                //     '<' => {
                //         stack.push('<');
                //     }
                //     '{' => {
                //         stack.push('{');
                //     }
                //     '(' => {
                //         stack.push('(');
                //     }
                //     '[' => {
                //         stack.push('<');
                //     }
                // }
            }
            0
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::part1;

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
}
