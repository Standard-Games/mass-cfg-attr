pub use mass_cfg_attr::*;

#[cfg(test)]
mod tests {
    mod try_build {
        use trybuild::TestCases;

        #[test]
        fn everything() {
            let test_case = TestCases::new();
            test_case.pass("tests/passing/*.rs");
            test_case.compile_fail("tests/failing_with_useful_errors/*.rs");

            // These tests do not test mass_cfg_attr directly but make sure that the passing tests
            // only pass _because_ of mass_cfg_attr. As the output varies by version of Rust, we
            // don't run them on older versions
            #[cfg(version(1.67))]
            test_case.compile_fail("tests/failing_no_mass_cfg_attr/*.rs");
            #[cfg(not(version(1.67)))]
            eprintln!("failing_no_mass_cfg_attr not run, re-run tests on Rust 1.67 or above");
        }
    }
}
