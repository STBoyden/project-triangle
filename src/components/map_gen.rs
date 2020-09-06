use super::map::Map;
use std::fs::File;
use std::io::BufReader;

pub fn load_map(file_path: &str) -> Result<Map, String> {
    let file = File::open(file_path);

    if file.is_err() {
        return Err(format!("Could not find file {}", file_path));
    }

    let file = file.ok().unwrap();
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).unwrap())
}
