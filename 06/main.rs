use std::fs;

fn part_one(input: &str) -> Option<i32> {
    let mut result = 0;

    for (i, _) in input.chars().enumerate() {
        let sel = &input[i..(i+4)];
        let mut letter_repeated = false;

        'sel_loop: for (j, l1) in sel.chars().enumerate() {
            for (k, l2) in sel.chars().enumerate() {
                if (j != k) && (l1 == l2) { // if same letter in different index, stop looking and go to the next letter
                    letter_repeated = true;
                    break 'sel_loop;
                }
            }
        }

        if !letter_repeated {
            result = (i as i32) + 4;
            break;
        }
    }

    Some(result)
}

fn part_two(input: &str) -> Option<i32> {
    let mut result = 0;

    for (i, _) in input.chars().enumerate() {
        let sel = &input[i..(i+14)];
        let mut letter_repeated = false;

        'sel_loop: for (j, l1) in sel.chars().enumerate() {
            for (k, l2) in sel.chars().enumerate() {
                if (j != k) && (l1 == l2) { // if same letter in different index, stop looking and go to the next letter
                    letter_repeated = true;
                    break 'sel_loop;
                }
            }
        }

        if !letter_repeated {
            result = (i as i32) + 14;
            break;
        }
    }

    Some(result)
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
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));

    }

}
