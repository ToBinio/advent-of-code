use std::fs;

pub fn get_string_vec(file_path: String) -> Vec<String> {
    let file = fs::read_to_string(&file_path).expect(&*format!("File {} not found", file_path));

    file.lines()
        .map(|line| line.to_string())
        .collect()
}