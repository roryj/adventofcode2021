use std::fmt::Write;

fn print_grid(grid: &Vec<Vec<u32>>) {
    println!("[");
    grid.iter().for_each(|row| println!("    {:?}\n", row));
    println!("]");
}

pub fn part1(input: Vec<String>) -> u32 {
    let lines: Vec<(u32, u32, u32, u32)> = input
        .iter()
        .map(|s| {
            let split: Vec<u32> = s
                .split("->")
                .flat_map(|s| s.split(",").map(|s2| s2.trim().parse::<u32>().unwrap()))
                .collect();

            if split.len() != 4 {
                panic!("Expected exactly 4 items. {:?}", split)
            }

            (split[0], split[1], split[2], split[3])
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;

    for l in lines.iter() {
        let curr_biggest_x = l.0.max(l.2) + 1;
        let curr_biggest_y = l.1.max(l.3) + 1;

        if curr_biggest_x > max_x {
            max_x = curr_biggest_x;
        }

        if curr_biggest_y > max_y {
            max_y = curr_biggest_y;
        }
    }

    println!("Found grid size of {}x{}", &max_x, &max_y);
    let mut grid: Vec<Vec<u32>> = vec![vec![0; max_x as usize]; max_y as usize];

    lines.iter().for_each(|(x1, y1, x2, y2)| {
        grid = add_to_grid(grid.clone(), *x1, *y1, *x2, *y2);
    });

    find_total_overlaps(grid)
}

pub fn part2(input: Vec<String>) -> u32 {
    let lines: Vec<(u32, u32, u32, u32)> = input
        .iter()
        .map(|s| {
            let split: Vec<u32> = s
                .split("->")
                .flat_map(|s| s.split(",").map(|s2| s2.trim().parse::<u32>().unwrap()))
                .collect();

            if split.len() != 4 {
                panic!("Expected exactly 4 items. {:?}", split)
            }

            (split[0], split[1], split[2], split[3])
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;

    for l in lines.iter() {
        let curr_biggest_x = l.0.max(l.2) + 1;
        let curr_biggest_y = l.1.max(l.3) + 1;

        if curr_biggest_x > max_x {
            max_x = curr_biggest_x;
        }

        if curr_biggest_y > max_y {
            max_y = curr_biggest_y;
        }
    }

    println!("Found grid size of {}x{}", &max_x, &max_y);
    let mut grid: Vec<Vec<u32>> = vec![vec![0; max_x as usize]; max_y as usize];

    lines.iter().for_each(|(x1, y1, x2, y2)| {
        grid = add_to_grid_v2(grid.clone(), *x1, *y1, *x2, *y2);
    });

    find_total_overlaps(grid)
}

fn add_to_grid_v2(mut grid: Vec<Vec<u32>>, x1: u32, y1: u32, x2: u32, y2: u32) -> Vec<Vec<u32>> {
    // println!("processing line {},{} to {},{}", x1, y1, x2, y2);
    // println!("before grid");
    // print_grid(&grid);
    if x1 == x2 {
        // println!("xs are the same");
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        for y in min_y..(max_y + 1) {
            grid[y as usize][x1 as usize] += 1;
        }
    } else if y1 == y2 {
        // println!("ys are the same");
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        for x in min_x..(max_x + 1) {
            grid[y1 as usize][x as usize] += 1;
        }
    } else {
        let x_change: i32 = match (x1, x2) {
            (x1, x2) if x1 > x2 => -1,
            (x1, x2) if x2 > x1 => 1,
            t => panic!("invalid x value {:?}", t),
        };
        let y_change: i32 = match (y1, y2) {
            (y1, y2) if y1 > y2 => -1,
            (y1, y2) if y2 > y1 => 1,
            t => panic!("invalid y value {:?}", t),
        };

        // we are doing a slope! We need to figure out if we are going up or down
        //
        // 2,0 -> 0,2
        // y2 > y1
        //
        // 0,0 -> 2, 2
        // y2 > y1
        //
        // 0 0 0
        // 0 0 0
        // 0 0 0
        //
        // x: 0, 1, 2
        // y: 2, 1, 0
        let mut curr_x = x1;
        let mut curr_y = y1;
        loop {
            grid[curr_y as usize][curr_x as usize] += 1;

            if curr_y == y2 || curr_x == x2 {
                break;
            }

            curr_y = (curr_y as i32 + y_change) as u32;
            curr_x = (curr_x as i32 + x_change) as u32;
        }
    }
    // println!("after grid");
    // print_grid(&grid);

    return grid.clone();
}

fn add_to_grid(mut grid: Vec<Vec<u32>>, x1: u32, y1: u32, x2: u32, y2: u32) -> Vec<Vec<u32>> {
    // println!("processing line {},{} to {},{}", x1, y1, x2, y2);
    // println!("before grid");
    // print_grid(&grid);
    if x1 == x2 {
        // println!("xs are the same");
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        for y in min_y..(max_y + 1) {
            grid[y as usize][x1 as usize] += 1;
        }
    } else if y1 == y2 {
        // println!("ys are the same");
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        for x in min_x..(max_x + 1) {
            grid[y1 as usize][x as usize] += 1;
        }
    }
    // println!("after grid");
    // print_grid(&grid);

    return grid.clone();
}

fn find_total_overlaps(grid: Vec<Vec<u32>>) -> u32 {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|overlap| match *overlap {
                    x if x > 1 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let result = part1(input);
        print!("Result: {}", result);
        assert_eq!(result, 5)
    }

    #[test]
    fn test_part2() {
        let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let result = part2(input);
        print!("Result: {}", result);
        assert_eq!(result, 12)
    }

    #[test]
    fn test_part2_simple() {
        let input = "2,0 -> 0,2
0,0 -> 2,2"
            .lines()
            .map(|s| s.to_string())
            .collect();

        let result = part2(input);
        print!("Result: {}", result);
        assert_eq!(result, 1)
    }
}
