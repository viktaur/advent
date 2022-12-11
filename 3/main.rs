use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader,BufRead};

fn get_priority(c: &char) -> u32 {
    if c.is_lowercase() {
        (*c as u32) - 96
    } else {
        (*c as u32) - 38
    }
}

fn part_one(file: &File) {
    let mut sum_priorities = 0;

    let rucksacks: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    
    for r in rucksacks {
        let bag_vec: Vec<char> = r.chars().collect(); 
        let (comp1, comp2) = bag_vec.split_at(bag_vec.len() / 2);

        let hs: HashSet<char> = comp1
            .iter()
            .filter(|&c| comp2.contains(c))
            .cloned()
            .collect();

        for i in hs {
            sum_priorities += get_priority(&i);
        }
    }

    println!("{}", sum_priorities);
}

fn part_two(file: &File) {
    let mut sum_priorities = 0;

    let rucksacks: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    for chunk in rucksacks.chunks(3) {
        let hs: HashSet<char> = chunk[0]
            .chars()
            .filter(|&c| chunk[1].contains(c) && chunk[2].contains(c))
            .collect();

        for i in hs {
            sum_priorities += get_priority(&i);
        }
    }

    println!("{}", sum_priorities);
}

fn main() {
    let file1 = File::open("src/input.txt").unwrap();
    part_one(&file1);

    let file2 = File::open("src/input.txt").unwrap();
    part_two(&file2);
}
