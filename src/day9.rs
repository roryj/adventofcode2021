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

pub fn part2(input: Vec<String>) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .iter()
        .map(|r| {
            r.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut result: Vec<u32> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let result: Vec<u32> = row
                .iter()
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

                    if y < grid.len() - 1 {
                        if grid[y + 1][x] <= *v {
                            return 0;
                        }
                    }

                    // print_grid(&grid);
                    get_basin(&grid, x, y).len() as u32
                })
                .collect::<Vec<u32>>(); // vec<u32>
            result
        })
        .collect::<Vec<u32>>();

    // reverse sort
    result.sort_by(|a, b| b.cmp(a));

    // result.iter().for_each(|r| println!("{:?}", r));

    result[..3].iter().fold(1, |accul, v| accul * v)
}

fn get_basin(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<u32> {
    let mut accessed_location: Vec<(usize, usize)> = Vec::new();
    let mut paths_to_check: Vec<(usize, usize)> = vec![(x, y)];
    let mut output: Vec<u32> = Vec::new();

    loop {
        let (curr_x, curr_y) = if let Some((x, y)) = paths_to_check.pop() {
            (x, y)
        } else {
            break;
        };

        if accessed_location.contains(&(curr_x, curr_y)) {
            continue;
        }

        // println!("getting value @ {},{}", curr_x, curr_y);
        // println!("{:?}, {:?}", output, accessed_location);

        let grid_value = grid[curr_y][curr_x];

        if grid_value == 9 {
            continue;
        }

        output.push(grid_value);
        accessed_location.push((curr_x, curr_y));

        if curr_x > 0 {
            paths_to_check.push((curr_x - 1, curr_y));
        }

        if curr_x < grid[0].len() - 1 {
            paths_to_check.push((curr_x + 1, curr_y));
        }

        if curr_y > 0 {
            paths_to_check.push((curr_x, curr_y - 1));
        }

        if curr_y < grid.len() - 1 {
            paths_to_check.push((curr_x, curr_y + 1));
        }
    }

    output
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    println!("[");
    grid.iter().for_each(|row| println!("    {:?}\n", row));
    println!("]");
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

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

    #[test]
    fn test_part2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678"
            .lines()
            .map(|s| s.to_string())
            .collect();
        let result = part2(input);
        assert_eq!(1134, result);
    }
}
