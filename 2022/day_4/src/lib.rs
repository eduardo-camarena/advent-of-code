use std::fs;

#[derive(Debug)]
pub struct ElfSection {
    pub first: i32,
    pub last: i32,
}

pub fn read_file() -> Vec<[ElfSection; 2]> {
    let input_file = fs::read_to_string("./input.txt").unwrap_or(String::from(""));
    return input_file
        .lines()
        .map(|line| {
            let ranges = line.split(",").collect::<Vec<&str>>();
            let first_elf = ranges[0]
                .split("-")
                .map(|val| val.parse().unwrap())
                .collect::<Vec<i32>>();

            let second_elf = ranges[1]
                .split("-")
                .map(|val| val.parse().unwrap())
                .collect::<Vec<i32>>();

            return [
                ElfSection {
                    first: first_elf[0],
                    last: first_elf[1],
                },
                ElfSection {
                    first: second_elf[0],
                    last: second_elf[1],
                },
            ];
        })
        .collect();
}

pub fn get_completely_overlapping_elfs(elf_pairs: Vec<[ElfSection; 2]>) -> i32 {
    let mut overlapping = 0;
    for elf_pair in elf_pairs.into_iter() {
        match elf_pair {
            [first_elf, second_elf] => {
                if first_elf.first <= second_elf.first && first_elf.last >= second_elf.last
                    || second_elf.first <= first_elf.first && second_elf.last >= first_elf.last
                {
                    overlapping += 1;
                }
            }
        }
    }

    return overlapping;
}

pub fn get_overlapping_elfs(elf_pairs: Vec<[ElfSection; 2]>) -> i32 {
    let mut overlapping = 0;
    for elf_pair in elf_pairs.into_iter() {
        match elf_pair {
            [first_elf, second_elf] => {
                if first_elf.first <= second_elf.first && first_elf.last >= second_elf.first
                    || second_elf.first <= first_elf.first && second_elf.last >= first_elf.first
                {
                    overlapping += 1;
                }
            }
        }
    }

    return overlapping;
}
