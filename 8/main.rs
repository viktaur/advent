use std::fs;

fn build_grid(input: &str) -> Vec<Vec<i32>> {

    let mut grid: Vec<Vec<i32>> = vec![];

    for (i, line) in input.lines().enumerate() {
        if line != "" {
            grid.insert(i, vec![]);
            for (j, c) in line.chars().enumerate() {
                let tree_height = c.to_digit(10).unwrap() as i32;
                grid[i].insert(j, tree_height);
            }
        }
    }

    grid
}

fn part_one(input: &str) -> Option<i32> {
    let grid = build_grid(input);
    let max_i = grid.len();
    let max_j = grid[0].len();

    let mut visible = 0;

    for i in 0..max_i {
        for j in 0..max_j {
            // if in edge
            if i == 0 || i == (max_i - 1) || j == 0 || j == (max_j - 1) {
                // println!("Tree in pos i: {} j: {} is visible", i, j);
                visible += 1;
                // else if it's visible
            } else {
                let (mut vis_u, mut vis_d, mut vis_l, mut vis_r) = (true, true, true, true);

                // check UP (lower i, same j) and DOWN (higher i, same j)
                for k in 0..max_i {
                    if k < i && grid[k][j] >= grid[i][j] {
                        vis_u = false;
                    }

                    if k > i && grid[k][j] >= grid[i][j] {
                        vis_d = false;
                    }

                    if !vis_u && !vis_d {
                        break;
                    }
                }
                // check LEFT (same i, lower j) and RIGHT (same i, higher j)
                for k in 0..max_j {
                    if k < j && grid[i][k] >= grid[i][j] {
                        vis_l = false;
                    }

                    if k > j && grid[i][k] >= grid[i][j] {
                        vis_r = false;
                    }

                    if !vis_l && !vis_r {
                        break;
                    }
                }

                if vis_u || vis_d || vis_l || vis_r {
                    // println!("Tree in pos i: {} j: {} is visible", i, j);
                    visible += 1;
                }
            }

        }
    }

    Some(visible)
}

fn part_two(input: &str) -> Option<i32> {
    let grid = build_grid(input);
    let max_i = grid.len();
    let max_j = grid[0].len();

    let mut scenic_score: i32 = 0;

    // for every tree (excluding trees on edges, whose score will be 0)
    for i in 1..(max_i - 1) {
        for j in 1..(max_j - 1) {
            let (mut n_up, mut n_down, mut n_left, mut n_right) = (1, 1, 1, 1);

            // checking UP (lower i, same j)
            while grid[i - n_up][j] < grid[i][j] {
                if (i as i32) - (n_up as i32) == 0 {
                    break;
                } else {
                    n_up += 1;
                }
            }

            // checking DOWN (higher i, same j)
            while grid[i + n_down][j] < grid[i][j] {
                if i + n_down == max_i - 1 {
                    break;
                } else {
                    n_down += 1;
                }
            }


            // checking LEFT (same i, lower j)
            while grid[i][j - n_left] < grid[i][j] {
                if j - n_left == 0 {
                    break;
                } else {
                    n_left += 1;
                }
            }

            // checking RIGHT (same i, higher j)
            while grid[i][j + n_right] < grid[i][j] {
                if j + n_right == max_j - 1 {
                    break;
                } else {
                    n_right += 1;
                }
            }

            let this_score: i32 = (n_up * n_down * n_left * n_right) as i32;

            if this_score > scenic_score {
                scenic_score = this_score;
            }
        }
    }

    Some(scenic_score)
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file input.txt for part one");

    let result1 = part_one(&input).unwrap();
    let result2 = part_two(&input).unwrap();

    println!("{result1}");
    println!("{result2}");
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("src/test.txt").expect("Error reading file text.txt for part one");
        assert_eq!(part_one(&input), Some(21));
        assert_eq!(part_two(&input), Some(8));
    }

}


