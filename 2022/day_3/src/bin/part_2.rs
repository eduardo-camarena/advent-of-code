use day_3::{get_group_badges, get_priotity_sum, read_file};

fn main() {
    let file = read_file();
    let group_badges = get_group_badges(file);
    let priority_sum = get_priotity_sum(&group_badges);

    println!("{}", priority_sum);
}
