#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

#[must_use]
struct ThreeInts {
    #[deprecated]
    first: i16,
    #[deprecated = "testing key-value"]
    second: i8,
    #[deprecated(since = "0.1.0", note = "this is a more complex example")]
    third: i32
}

fn main() {
    // Violates unused_must_use and deprecated without mass_cfg_attr
    ThreeInts {
        first: 1,
        second: 2,
        third: 3
    };
}
