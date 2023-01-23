pub use mass_cfg_attr::*;

#[cfg(test)]
mod tests {
    mod try_build {
        use trybuild::TestCases;

        #[test]
        fn everything() {
            let test_case = TestCases::new();
            test_case.pass("tests/passing/*.rs");
            test_case.compile_fail("tests/failing_no_mass_cfg_attr/*.rs");
        }
    }
}
