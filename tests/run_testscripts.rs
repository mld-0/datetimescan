
#[cfg(test)]
mod call_test_script {

    //use std::process::Command;
    //use std::env;
    //use std::path::Path;
    //use std::str;
    //#[test]
    //fn run() 
    //{
    //    let project_directory = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    //    let script_path = format!("{}/tests/datetimescan.sh", project_directory);
    //    if cfg!(debug_assertions) {
    //        std::env::set_var("RUST_TEST_RELEASE", "false");
    //    } else {
    //        std::env::set_var("RUST_TEST_RELEASE", "true");
    //    }
    //    assert!(Path::new(&script_path).exists(), "Test script=({}) does not exist", script_path);
    //    let output = Command::new("bash")
    //        .arg(&script_path)
    //        .output()
    //        .expect("failed to execute script");
    //    let _output_stdout = str::from_utf8(&output.stdout).unwrap();
    //    let output_stderr = str::from_utf8(&output.stderr).unwrap();
    //    println!("{}", output_stderr);
    //    assert!(output.status.success());
    //}
}
