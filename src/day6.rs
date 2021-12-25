use std::fmt::Debug;

#[derive(Clone)]
struct LanternFish {
    days_left: u8,
}

impl LanternFish {
    fn new(start_days: Option<u8>) -> Self {
        let days_left = match start_days {
            Some(d) => d,
            None => 8,
        };

        Self { days_left }
    }

    fn tick(&mut self) -> bool {
        if self.days_left == 0 {
            self.days_left = 6;
            return true;
        }

        self.days_left -= 1;

        false
    }
}

impl Debug for LanternFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.days_left))
    }
}

pub fn part1(input: Vec<u8>, num_days: u64) -> u64 {
    let mut fishes: Vec<LanternFish> = input.iter().map(|d| LanternFish::new(Some(*d))).collect();

    println!("Starting fishes: {:?}", &fishes);

    for i in 0..num_days {
        let mut new_fishes: Vec<LanternFish> = Vec::new();

        fishes.iter_mut().for_each(|f| match f.tick() {
            true => new_fishes.push(LanternFish::new(None)),
            false => {}
        });

        fishes.append(&mut new_fishes);

        // println!("Day {}, {} fishes", i, fishes.len());

        // println!("Day {}: {:?}", i, &fishes);
    }

    fishes.len() as u64
}

/// Stores how many days the fishes need to reproduce
#[derive(Debug)]
struct SeaOfFishes {
    fish_reproductions: [u64; 9],
}

impl SeaOfFishes {
    fn from_input(days: Vec<u8>) -> Self {
        let mut fishes = [0 as u64; 9];

        // add each fishes' current reproduction days (init)
        for d in days {
            fishes[d as usize] += 1;
        }

        return SeaOfFishes {
            fish_reproductions: fishes,
        };
    }

    /// Lets one day pass
    fn tick_a_day(&mut self) {
        // remember how many fishes will reproduce
        let reproducing_fishes = self.fish_reproductions[0];

        // Move each fish one slot (day) forwars
        for i in 0 as usize..=7 {
            self.fish_reproductions[i] = self.fish_reproductions[i + 1];
        }

        // new fishes will need 8 days to reproduce
        self.fish_reproductions[8] = reproducing_fishes;

        // Also the same fishes which reproduced will reproduce again in 6 days
        self.fish_reproductions[6] += reproducing_fishes;
    }

    /// just a wrapper using [tick_a_day] multiple times
    fn tick_n_days(&mut self, days: usize) {
        for _ in 0..days {
            self.tick_a_day();
        }
    }

    /// how many fishes we have in the pool?
    fn count_the_pool(&self) -> u64 {
        self.fish_reproductions.iter().sum()
    }
}

pub fn part2(input: Vec<u8>) -> u64 {
    let mut field = SeaOfFishes::from_input(input);
    field.tick_n_days(256);
    field.count_the_pool()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "3,4,3,1,2"
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let result = part1(input, 18);

        assert_eq!(result, 26);
    }

    #[test]
    fn test_part1_bigger() {
        let input = "3,4,3,1,2"
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let result = part1(input, 80);

        assert_eq!(result, 5934);
    }

    #[test]
    fn test_part1_biggest() {
        let input = "3,4,3,1,2"
            .split(",")
            .map(|s| s.parse::<u8>().unwrap())
            .collect();

        let result = part2(input);

        assert_eq!(result, 26984457539);
    }
}
