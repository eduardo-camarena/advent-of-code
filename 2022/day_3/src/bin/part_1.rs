use day_3::{get_priotity_sum, get_repeated_items_part_one, read_file};

fn main() {
    let input_file = read_file();
    let repeated_items = get_repeated_items_part_one(input_file);
    let priority_sum = get_priotity_sum(&repeated_items);

    println!("{}", priority_sum);
}
