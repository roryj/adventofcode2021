pub fn part1(depths: Vec<u32>) -> u32 {
    let mut increase_count = 0;

    let mut prev_depth = None;

    for d in depths.iter() {
        match prev_depth {
            Some(p) => {
                if p < d {
                    increase_count += 1;
                }
            }
            None => {}
        }

        prev_depth = Some(d)
    }

    increase_count
}

pub fn part2(depths: Vec<u32>) -> u32 {
    let mut increase_count = 0;
    let mut prev_sliding_window_sum = None;

    for (index, _) in depths.iter().enumerate() {
        if index + 2 >= depths.len() {
            break;
        }

        let curr_sum = depths[index..index + 3].iter().fold(0, |a, &b| a + b);
        // println!(
        //     "calculated sum for {:?}+{:?}+{:?} = {}",
        //     depths.get(index),
        //     depths.get(index + 1),
        //     depths.get(index + 2),
        //     curr_sum
        // );

        if let Some(p) = prev_sliding_window_sum {
            if curr_sum > p {
                increase_count += 1;
            }
        }

        prev_sliding_window_sum = Some(curr_sum);
    }

    increase_count
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    #[test]
    fn test_part1() {
        let depths = vec![100, 200, 300, 299, 288, 500];

        let result = part1(depths);

        assert_eq!(result, 3)
    }

    #[test]
    fn test_part2() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = part2(depths);

        assert_eq!(result, 5)
    }
}
