#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

use mass_cfg_attr::mass_cfg_attr;

#[mass_cfg_attr(test, [must_use, deprecated])]
#[must_use]
enum Enum {
    #[deprecated]
    NotOk,

    Ok,
}

fn main() {
    // Violates unused_must_use and deprecated without mass_cfg_attr
    Enum::NotOk;
}
