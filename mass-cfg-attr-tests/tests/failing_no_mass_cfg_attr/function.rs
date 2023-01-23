#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

#[deprecated]
#[must_use]
fn get_random_number() -> u8 {
    4 // chosen by fair dice roll
}

fn main() {
    // Violation of unused_must_use if not removed by mass_cfg_attr
    get_random_number();
}
