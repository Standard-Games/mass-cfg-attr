#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

use mass_cfg_attr::mass_cfg_attr;

#[mass_cfg_attr(test, must_use)]
#[must_use]
struct Struct();

fn main() {
    // Violation of unused_must_use if not removed by mass_cfg_attr
    Struct();
}
