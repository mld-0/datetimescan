use std::path::PathBuf;
use std::env;

//  Notes:
//  2023-06-12T22:21:14AEST should be called (something like) 'get_testfile_paths'?
//  2023-06-12T22:21:25AEST place in a module ((presumedly) they're not because there was an issuing trying to use them from a module?

#[cfg(test)]
pub fn get_path_empty() -> String {
    get_test_data_file("empty.txt")
}

#[cfg(test)]
pub fn get_path_nodatetimes() -> String {
    get_test_data_file("noDatetimes.txt")
}

#[cfg(test)]
pub fn get_path_textwithisodatetimes1() -> String {
    get_test_data_file("textWithIsoDatetimes-1.txt")
}

#[cfg(test)]
pub fn get_path_textwithisodatetimes2() -> String {
    get_test_data_file("textWithIsoDatetimes-2.txt")
}

#[cfg(test)]
pub fn get_path_worklogscrambledsamples() -> String {
    get_test_data_file("worklog.scrambled.samples.txt")
}

#[cfg(test)]
pub fn get_path_partialdatetimes() -> String {
    get_test_data_file("partialDatetimes.txt")
}

#[cfg(test)]
fn get_test_data_file(filename: &str) -> String {
    let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("tests/data")
        .join(filename);
    if !path.exists() {
        panic!("Test data file does not exist: {:?}", path);
    }    
    path.to_str().expect("Failed to convert path to string").to_owned()
}
