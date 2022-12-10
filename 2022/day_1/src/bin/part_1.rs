use day_1::{get_max_calories, parse_input};

fn main() {
    let parsed_input = parse_input();
    let max_calories = get_max_calories(parsed_input);
    println!("{}", max_calories);
}
