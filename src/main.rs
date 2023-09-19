fn main() {
    let args : Vec<String> = std::env::args().collect();
    let string = args.get(1).expect("r_grep expected string argument");
    let file_path = args.get(2).expect("r_grep expected file argument");
    std::fs::read_to_string(file_path)
        .expect("could not read file")
        .as_str()
        .split("\n")
        .enumerate()
        .filter(|(_, line)| line.contains(string))
        .for_each(|(index, line)| println!("({}): {}", index + 1, line))
}