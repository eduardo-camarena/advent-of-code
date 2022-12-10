use day_2::{get_score_part_one, read_file};

fn main() {
    let input_file = read_file();
    let score = get_score_part_one(input_file);
    println!("{}", score);
}
