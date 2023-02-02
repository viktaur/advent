use std::fs;

fn part_one(input: &str) -> Option<i32> {

    let mut grid: Vec<Vec<char>> = vec![];

    let mut n_rows = 0;
    let mut n_cols = 0;

    for (i, line) in input.lines().enumerate() {
        grid.insert(i, vec![]);
        n_rows = i;

        for (j, c) in line.chars().enumerate() {
            grid[i].insert(j, c);
            n_cols = j;
        }
    }

    n_rows += 1;
    n_cols += 1;

    let mut trees = 0;

    let mut i = 0;
    let mut j = 0;

    while i < n_rows {
        let c = grid[i][j];

        match c {
            '#' => trees += 1,
            _ => ()
        }

        i += 1;
        j = (j + 3) % n_cols ;
    }

    Some(trees)
}

fn calculate_trees(input: &str, right: usize, down: usize) -> Option<i64> {

    let mut grid: Vec<Vec<char>> = vec![];

    let mut n_rows = 0;
    let mut n_cols = 0;

    for (i, line) in input.lines().enumerate() {
        grid.insert(i, vec![]);
        n_rows = i;

        for (j, c) in line.chars().enumerate() {
            grid[i].insert(j, c);
            n_cols = j;
        }
    }

    n_rows += 1;
    n_cols += 1;

    let mut trees = 0;

    let mut i = 0;
    let mut j = 0;

    while i < n_rows {
        let c = grid[i][j];

        match c {
            '#' => trees += 1,
            _ => ()
        }

        i += down;
        j = (j + right) % n_cols ;
    }

    Some(trees)
}

fn part_two(input: &str) -> Option<i64> {
    Some(
        calculate_trees(&input, 1, 1).unwrap() *
        calculate_trees(&input, 3, 1).unwrap() *
        calculate_trees(&input, 5, 1).unwrap() *
        calculate_trees(&input, 7, 1).unwrap() *
        calculate_trees(&input, 1, 2).unwrap()
    )
}


fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Error reading file");
    println!("{}", part_one(&input).unwrap());
    println!("{}", part_two(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("src/example.txt").expect("Error reading file");
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/example.txt").expect("Error reading file");
        assert_eq!(part_two(&input), Some(336));
    }

}