use std::fs;
use prse::parse;

fn part_one(input: &str) -> Option<i32> {
    
    let mut valid_passwords = 0;

    for line in input.lines() {
        let (min, max, letter, password): (i32, i32, char, &str) = parse!(line, "{}-{} {}: {}");

        let mut count = 0;

        for c in password.chars() {
            if c == letter {
                count += 1;
            } 
        }

        if count >= min && count <= max {
            valid_passwords += 1;
        }
    }

    Some(valid_passwords)
}

fn part_two(input: &str) -> Option<i32> {
    
    let mut valid_passwords = 0;

    for line in input.lines() {
        let (pos1, pos2, letter, password): (usize, usize, char, &str) = parse!(line, "{}-{} {}: {}");

        let mut count = 0;
        
        for (i, c) in password.chars().enumerate() {
            if (i+1 == pos1 || i+1 == pos2) && c == letter {
                count += 1;
            } 
        }

        if count == 1 {
            valid_passwords += 1;
        }
    }

    Some(valid_passwords)
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
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/example.txt").expect("Error reading file");
        assert_eq!(part_two(&input), Some(1));
    }

}