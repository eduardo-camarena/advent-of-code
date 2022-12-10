use std::fs;

pub fn read_file() -> Vec<[char; 2]> {
    let input_file = fs::read_to_string("./input.txt").unwrap_or(String::from(""));
    input_file
        .lines()
        .map(|line| [line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()])
        .collect()
}

fn get_winner_part_one(elfs_play: char, my_play: char) -> i32 {
    // X, A: ROCK
    // Y, B: PAPER
    // Z, C: SCISSORS
    match my_play {
        'X' => match elfs_play {
            'B' => 1,
            'A' => 4,
            _ => 7,
        },
        'Y' => match elfs_play {
            'C' => 2,
            'B' => 5,
            _ => 8,
        },
        _ => match elfs_play {
            'A' => 3,
            'C' => 6,
            _ => 9,
        },
    }
}

pub fn get_score_part_one(input: Vec<[char; 2]>) -> i32 {
    let mut score = 0;
    for game in input.iter() {
        score += get_winner_part_one(game[0], game[1]);
    }

    score
}

fn get_winner_part_two(elfs_play: char, desired_outcome: char) -> i32 {
    match elfs_play {
        'A' => match desired_outcome {
            'X' => 3,
            'Y' => 4,
            _ => 8,
        },
        'B' => match desired_outcome {
            'X' => 1,
            'Y' => 5,
            _ => 9,
        },
        _ => match desired_outcome {
            'X' => 2,
            'Y' => 6,
            _ => 7,
        },
    }
}

pub fn get_score_part_two(input: Vec<[char; 2]>) -> i32 {
    let mut score = 0;
    for game in input.iter() {
        score += get_winner_part_two(game[0], game[1]);
    }

    score
}
