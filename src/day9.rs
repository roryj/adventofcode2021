pub fn part1(input: Vec<String>) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .iter()
        .map(|r| {
            r.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, v)| {
                    // curent location is @ x,y, we need to look at x-1, x+1, y-1, y+1 to check if it is the lowest
                    // println!("@ ({},{}) => {}", x, y, v);
                    // println!("actual @ {:?}", grid[y][x]);
                    if x > 0 {
                        if grid[y][x - 1] <= *v {
                            return 0;
                        }
                    }

                    if x < grid[y].len() - 1 {
                        if grid[y][x + 1] <= *v {
                            return 0;
                        }
                    }

                    if y > 0 {
                        if grid[y - 1][x] <= *v {
                            return 0;
                        }
                    }

                    // print_grid(&grid);
                    if y < grid.len() - 1 {
                        if grid[y + 1][x] <= *v {
                            return 0;
                        }
                    }

                    *v + 1
                })
                .sum::<u32>()
        })
        .sum()
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    println!("[");
    grid.iter().for_each(|row| println!("    {:?}\n", row));
    println!("]");
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn test_part1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678"
            .lines()
            .map(|s| s.to_string())
            .collect();
        let result = part1(input);
        assert_eq!(15, result);
    }
}
