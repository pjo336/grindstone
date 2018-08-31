use std::fs::File;
use std::io::Read;

pub fn get_config_file() -> String {
    let mut s = String::new();
    let mut f: File = locate_grindstone_file();
    f.read_to_string(&mut s).unwrap();
    s
}

/**
 * Locate the root level grindstone json file.
 * If no file is found, the program should panic and exit with a concise message
 *
 * TODO: Support non json formats (yml? toml?)
 * TODO: Locate files in other locations besides project root
 */
fn locate_grindstone_file() -> File {
    match File::open("grindstone.json") {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening grindstone file: {:?}", error),
    }
}