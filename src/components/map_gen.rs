use super::map::Map;
use serde_json::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_map<P: AsRef<Path>>(file_path: P) -> Result<Map> {
    let file = File::open(file_path);

    if file.is_err() {
        return Ok(Map {
            map_name: "".to_string(),
            spawn_point: (0, 0),
            objects: vec![],
            entities: vec![],
        });
    }

    let file = file.ok().unwrap();
    let reader = BufReader::new(file);
    let json = serde_json::from_reader(reader);

    println!("{:?}", json);

    json
}
