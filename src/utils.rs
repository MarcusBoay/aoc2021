use std::{env, fs};

/// Reads data from file in tests/<srcname>/<FILE_NAME>.txt specified by second
/// argument and returns it as Vec<String>.
/// Panics if no file path is provided OR file path is provided is invalid.
#[allow(dead_code)]
pub fn fast_get_file_data_as_vec() -> Vec<String> {
    fast_get_file_data_as_string()
        .trim() // Remove blank space at end of line.
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>()
}

/// Reads data from file in tests/<srcname>/<FILE_NAME>.txt specified by second
/// argument and returns it as String.
/// Panics if no file path is provided OR file path is provided is invalid.
#[allow(dead_code)]
pub fn fast_get_file_data_as_string() -> String {
    let args = env::args().collect::<Vec<String>>();
    let bin_name = &args[0]
        .split('/')
        .map(String::from)
        .collect::<Vec<String>>()
        .last()
        .unwrap()
        .to_owned();
    if env::args().len() != 2 {
        panic!("Usage: {} tests/{bin_name}/file", &args[0]);
    }

    let filepath =
        String::from("tests/") + bin_name + &String::from("/") + &args[1] + &String::from(".txt");
    fs::read_to_string(&filepath)
        .unwrap_or_else(|_| panic!("Unable to read file {}.", filepath.to_owned()))
}

/// Reads data from file specified by second argument and returns it as Vec<String>.
/// Panics if no file path is provided OR file path is provided is invalid.
#[allow(dead_code)]
pub fn get_file_data_as_vec() -> Vec<String> {
    get_file_data_as_string()
        .trim() // Remove blank space at end of line.
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>()
}

/// Reads data from file specified by second argument and returns it as String.
/// Panics if no file path is provided OR file path is provided is invalid.
#[allow(dead_code)]
pub fn get_file_data_as_string() -> String {
    let args = env::args().collect::<Vec<String>>();
    if env::args().len() != 2 {
        panic!("Usage: {} path/to/file", &args[0]);
    }

    let filepath = &args[1];
    fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Unable to read file {}.", filepath.to_owned()))
}
