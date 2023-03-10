#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

use mass_cfg_attr::mass_cfg_attr;

struct Struct();

#[mass_cfg_attr(test, [must_use, deprecated])]
#[deprecated]
impl Struct {
    #[must_use]
    pub fn get_random_number() -> u8 {
        4 // chosen by fair dice roll
    }
}

fn main() {
    // Violation of unused_must_use if not removed by mass_cfg_attr
    Struct::get_random_number();
}
