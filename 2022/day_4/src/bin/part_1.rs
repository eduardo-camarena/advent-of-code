use day_4::{get_completely_overlapping_elfs, read_file};

fn main() {
    let file = read_file();
    let overlapping = get_completely_overlapping_elfs(file);

    println!("{}", overlapping);
}
