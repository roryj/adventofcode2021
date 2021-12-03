pub fn part1(moves: Vec<String>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;

    moves.iter().for_each(|m| {
        let pieces: Vec<&str> = m.split(" ").collect();

        let distance = pieces[1].parse::<u32>().unwrap();

        match pieces[0] {
            "forward" => horizontal += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            v => panic!("unexpected value: {}", v),
        }
    });

    horizontal * depth
}

pub fn part2(moves: Vec<String>) -> u32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    moves.iter().for_each(|m| {
        let pieces: Vec<&str> = m.split(" ").collect();

        let units = pieces[1].parse::<u32>().unwrap();

        match pieces[0] {
            "forward" => {
                horizontal += units;
                depth += units * aim;
            }
            "down" => aim += units,
            "up" => aim -= units,
            v => panic!("unexpected value: {}", v),
        }
    });

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            .lines()
            .map(|s| String::from(s))
            .collect();

        let result = part1(input);

        assert_eq!(150, result)
    }

    #[test]
    fn test_part2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            .lines()
            .map(|s| String::from(s))
            .collect();

        let result = part2(input);

        assert_eq!(900, result)
    }
}
