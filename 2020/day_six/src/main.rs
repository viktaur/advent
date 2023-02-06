use std::fs;
use std::collections::HashSet;

fn part_one(input: &str) -> Option<i32> {

    let mut n_questions = 0;
    let mut current_group_questions: HashSet<char> = HashSet::new();

    for line in input.lines() {
        if line == "" {
            n_questions += current_group_questions.len() as i32;
            current_group_questions = HashSet::new();
        }

        for c in line.chars() {
            current_group_questions.insert(c);
        }
    }
    
    n_questions += current_group_questions.len() as i32;

    Some(n_questions)
}

fn part_two(input: &str) -> Option<i32> {
    
    let mut n_questions = 0;
    let mut current_group: Vec<HashSet<char>> = vec![];

    for line in input.lines() {
        if line == "" {
            let mut iter = current_group.clone().into_iter();
            let mut current_individual = iter.next().unwrap(); // assign it to first element

            for i in iter {
                current_individual = intersect_sets(current_individual.clone(), i.clone());
            }

            n_questions += current_individual.len() as i32;
            current_group = vec![];
        } else {
            let mut individual_questions = HashSet::new();

            for c in line.chars() {
                individual_questions.insert(c);
            }

            current_group.push(individual_questions);
        }

    }

    if !current_group.is_empty() {
        let mut iter = current_group.clone().into_iter();
        let mut current_individual = iter.next().unwrap(); // assign it to first element

        for i in iter {
            current_individual = intersect_sets(current_individual.clone(), i.clone());
        }

        n_questions += current_individual.len() as i32;
    }

    Some(n_questions)
}

fn intersect_sets(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Erorr reading file");
    println!("{}", part_one(&input).unwrap());
    println!("{}", part_two(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("src/example.txt").expect("Error reading file");
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("src/example.txt").expect("Error reading file");
        assert_eq!(part_two(&input), Some(6));
    }
}

