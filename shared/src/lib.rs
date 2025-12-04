use std::env;

pub fn get_input(day: u8) -> String {
    if let Err(e) = dotenvy::dotenv() {
        panic!("Error loading .env file: {}", e);
    }
    let base_path = match env::var("CWD") {
        Ok(path) => std::path::PathBuf::from(path),
        Err(_) => panic!("Could not find CWD in .env file"),
    };

    match std::fs::read_to_string(base_path.join("inputs").join(format!("day{day}.txt"))) {
        Ok(input) => input,
        Err(e) => panic!("Could not read input file: {}", e),
    }
}
