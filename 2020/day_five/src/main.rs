use std::fs;
use std::cmp::max;
use std::collections::BinaryHeap;

fn calc_id(code: &str) -> Option<i32> {
    let mut min_row = 0;
    let mut max_row = 127;

    let mut min_col = 0;
    let mut max_col = 7;

    for c in code.chars() {
        match c {
            'F' => max_row = (min_row + (max_row - 1)) / 2,
            'B' => min_row = (min_row + (max_row + 1)) / 2,
            'L' => max_col = (min_col + (max_col - 1)) / 2,
            'R' => min_col = (min_col + (max_col + 1)) / 2,
            _ => return None
        }
    }

    Some(max_row * 8 + max_col)
}

fn part_one(input: &str) -> Option<i32> {
    let mut highest_seat_id = 0;

    for line in input.lines() {
        highest_seat_id = max(highest_seat_id, calc_id(line).unwrap());
    }

    Some(highest_seat_id)
}

fn part_two(input: &str) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    
    for line in input.lines() {
        heap.push(calc_id(line).unwrap());
    }

    while !heap.is_empty() {
        let a = heap.pop().unwrap();
        let b = heap.peek().unwrap();

        if a - b == 2 {
            return Some(b+1);
        }
    }

    None
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
        assert_eq!(part_one(&input), Some(820));
    }
}
