pub fn part1(input: Vec<u32>) -> u32 {
    let min_value = input.iter().min().unwrap().to_owned();
    let max_value = input.iter().max().unwrap().to_owned();

    let result = (min_value..max_value)
        .into_iter()
        .map(|curr_value| {
            input
                .iter()
                .map(|i| (*i as i32 - curr_value as i32).abs())
                .sum::<i32>() as u32
        })
        .min()
        .unwrap();

    result
}

pub fn part2(input: Vec<u32>) -> u32 {
    let min_value = input.iter().min().unwrap().to_owned();
    let max_value = input.iter().max().unwrap().to_owned();

    let result = (min_value..max_value)
        .into_iter()
        .map(|curr_value| {
            input
                .iter()
                .map(|i| {
                    let diff = (*i as i32 - curr_value as i32).abs();

                    (1..diff + 1).into_iter().map(|v| v as u32).sum::<u32>()
                })
                .sum::<u32>()
        })
        .min()
        .unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "16,1,2,0,4,2,7,1,2,14"
            .split(",")
            .map(|i| i.parse::<u32>().unwrap())
            .collect();

        let result = part1(input);
        assert_eq!(37, result);
    }

    #[test]
    fn test_part2() {
        let input = "16,1,2,0,4,2,7,1,2,14"
            .split(",")
            .map(|i| i.parse::<u32>().unwrap())
            .collect();

        let result = part2(input);
        println!("result: {}", result);
        assert_eq!(168, result);
    }
}
