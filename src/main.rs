use std::{fs, env};

fn get_arguments() -> (String , String) {
    let args : Vec<String> = env::args().collect();
    let error = "r_grep expected string and file path arguments";
    (args.get(1).expect(error).clone(), args.get(2).expect(error).clone())
}

fn search_file() -> Vec<String> {
    let (string, file_path) = get_arguments();
    fs::read_to_string(file_path)
        .expect("could not read file")
        .as_str()
        .split("\n")
        .enumerate()
        .filter(|(_, line)| line.contains(&string))
        .map(|(index, line)| format!("({}): {}", index + 1, line))
        .collect()
}

fn main() {
    search_file()
        .iter()
        .for_each(|line| println!("{}", line,))
}
