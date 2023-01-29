pub use mass_cfg_attr::*;

#[cfg(test)]
mod tests {
    mod try_build {
        use std::{fs, io};
        use std::fs::{DirEntry, File};
        use std::io::BufRead;
        use std::path::Path;
        use trybuild::TestCases;
        use std::io::Write;

        #[test]
        fn test_the_tests() {
            // Copy each file from mass-cfg-attr-tests/tests/passing
            let source_path = Path::new("../mass-cfg-attr-tests/tests/passing");
            let test_path = Path::new("./tests/failing_no_mass_cfg_attr");

            fs::read_dir(source_path)
                .expect("Could not read dir")
                .map(|dir_result| dir_result.expect("Could not read dir"))
                .filter(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .expect("Could not read entry name")
                        .ends_with(".rs")
                })
                .for_each(|old_test: DirEntry| {
                    let new_test = test_path.join(old_test.file_name());
                    let _ = fs::remove_file(&new_test);
                    let mut new_test_file = File::create(&new_test).expect("Could not create file");

                    let old_test_file = File::open(old_test.path()).expect("Could not read file");
                    let reader = io::BufReader::new(old_test_file).lines();

                    for line in reader {
                        let line = line.expect("Could not read line");
                        if !line.contains("mass_cfg_attr") {
                            new_test_file.write(line.as_bytes()).expect("Could not write to file");
                            new_test_file.write("\n".as_bytes()).expect("Could not write to file");
                        }
                    }
                });

            // Remove mass-cfg-attr

            // Make sure the tests fail
            let test_case = TestCases::new();
            test_case.compile_fail("tests/failing_no_mass_cfg_attr/*.rs");
        }
    }
}
