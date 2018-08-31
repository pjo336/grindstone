use std::fs::File;
use std::io::Read;


const FILE_NAME: &str = "grindstone.json";

pub fn get_config_file() -> String {
    let mut s = String::new();
    let mut f: File = locate_grindstone_file(FILE_NAME);
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
fn locate_grindstone_file(file_name: &str) -> File {
    match File::open(file_name) {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening grindstone file: {:?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Move to integration test suite to use a created file
    #[test]
    fn test_get_config_file() {
        let s = get_config_file();
        assert!(s.len() > 0);
    }

    #[test]
    fn test_locate_grindstone_file() {
        let f = locate_grindstone_file(FILE_NAME);
        assert!(f.bytes().count() > 0);
    }
    #[test]
    #[should_panic]
    fn test_locate_grindstone_file_not_found() {
        locate_grindstone_file("notafile.json");
    }

}