use std::{fs, vec};

pub fn read_file() -> Vec<String> {
    let input_file = fs::read_to_string("./input.txt").unwrap_or(String::from(""));
    return input_file.lines().map(|line| String::from(line)).collect();
}

pub fn get_priotity_sum(chars: &Vec<char>) -> i64 {
    let mut priority_sum: i64 = 0;

    for char in chars.into_iter() {
        let x = match char {
            x if x.is_lowercase() => *x as i64 - 96,
            x if x.is_uppercase() => *x as i64 - 38,
            _ => 0,
        };

        priority_sum += x;
    }

    return priority_sum;
}

pub fn get_repeated_items_part_one(file: Vec<String>) -> Vec<char> {
    let mut repeated_items: Vec<char> = vec![];
    for line in file.into_iter() {
        let max_i = line.len() / 2;
        let mut i = 0;
        while i < max_i {
            let mut j = max_i;
            while j < line.len() {
                if line.chars().nth(i).unwrap() == line.chars().nth(j).unwrap() {
                    repeated_items.push(line.chars().nth(j).unwrap());
                    i = max_i;
                    j = line.len();
                }
                j += 1;
            }
            i += 1;
        }
    }

    return repeated_items;
}

pub fn get_group_badges(file: Vec<String>) -> Vec<char> {
    let mut i: usize = 0;
    let mut badges: Vec<char> = vec![];

    while i < file.len() {
        let mut j = 0;
        while j < file[i].len() {
            let current_char = file[i].chars().nth(j).unwrap();
            let second_elf = file[i + 1].find(current_char);
            let third_elf = file[i + 2].find(current_char);

            if second_elf.is_none() || third_elf.is_none() {
                j += 1;
            } else {
                badges.push(current_char);
                j = file[i].len();
            }
        }
        i += 3;
    }

    return badges;
}
