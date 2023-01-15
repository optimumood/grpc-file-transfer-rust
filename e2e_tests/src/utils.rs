use std::{fs, path::PathBuf};

pub fn compare_files(left: &PathBuf, right: &PathBuf) -> bool {
    let left_data = fs::read(left).expect("Failed to read left file");
    let right_data = fs::read(right).expect("Failed to read right file");

    left_data == right_data
}
