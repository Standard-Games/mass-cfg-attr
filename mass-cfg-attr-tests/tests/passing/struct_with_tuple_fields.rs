#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

use mass_cfg_attr::mass_cfg_attr;

#[mass_cfg_attr(test, [must_use, deprecated])]
#[must_use]
struct ThreeInts(
    #[deprecated]
    i16,
    #[deprecated = "testing key-value"]
    i8,
    #[deprecated(since = "0.1.0", note = "this is a more complex example")]
    i32,
);

fn main() {
    ThreeInts(
        1,
        2,
        3,
    );
}
