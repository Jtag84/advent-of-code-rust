use crate::lib::parser_commons::read_file_to_string;

pub fn parse_input(input_path: &str) -> Vec<String> {
    let file_string = read_file_to_string(input_path);
    file_string
        .lines()
        .into_iter()
        .map(|line| line.to_string())
        .collect()
}
