use std::fs;

pub fn parse_input() -> Vec<i32> {
    let input_file = fs::read_to_string("./input.txt").unwrap_or(String::from(""));
    let parsed_input = input_file
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
                .into_iter()
                .sum()
        })
        .collect::<Vec<i32>>();

    parsed_input
}

pub fn get_max_calories(calories_vec: Vec<i32>) -> i32 {
    let max_calories = calories_vec.into_iter().max().unwrap();

    max_calories
}

pub fn get_top_three_max_calories(mut calories_vec: Vec<i32>) -> i32 {
    let mut sum_of_max_calories = 0;
    calories_vec.sort_by(|a, b| b.cmp(a));

    for i in 0..3 {
        sum_of_max_calories += calories_vec[i];
        println!(
            "calories: {}, sum: {}",
            calories_vec[i], sum_of_max_calories
        );
    }

    sum_of_max_calories
}
