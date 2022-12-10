use day_1::{get_top_three_max_calories, parse_input};

fn main() {
    let parsed_input = parse_input();

    let top_three_max_calories = get_top_three_max_calories(parsed_input);

    println!("{}", top_three_max_calories);
}
