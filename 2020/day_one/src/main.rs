use std::fs;

fn part_one(input: &str) -> Option<i32> {

    for i in input.lines() {
        for j in input.lines() {
            
            let x: i32 = i.parse().unwrap();
            let y: i32 = j.parse().unwrap();

            if x + y == 2020 && x != y {
                return Some(x * y);
            } 
        }
    }

    None
}

fn part_two(input: &str) -> Option<i32> {

    for i in input.lines() {
        for j in input.lines() {
            for k in input.lines() {
                
                let x: i32 = i.parse().unwrap();
                let y: i32 = j.parse().unwrap();
                let z: i32 = k.parse().unwrap();

                if x + y + z == 2020 && x != y && y != z && x != z {
                    return Some(x * y * z);
                } 
            }
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
        assert_eq!(part_one(&input), Some(514579));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/example.txt").expect("Error reading file");
        assert_eq!(part_two(&input), Some(241861950));
    }

}