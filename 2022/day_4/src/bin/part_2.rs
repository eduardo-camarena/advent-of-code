use day_4::{get_overlapping_elfs, read_file};

fn main() {
    let file = read_file();

    println!("{}", get_overlapping_elfs(file));
}
