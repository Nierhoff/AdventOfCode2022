use std::env;
use std::fs;

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));
    println!("{}", filepath.to_str().unwrap());
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
