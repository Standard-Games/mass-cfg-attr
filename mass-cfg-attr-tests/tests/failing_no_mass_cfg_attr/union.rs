#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

#[must_use]
union Union {
    #[deprecated]
    f: f64,

    u: u64,
}

fn main() {
    // Violates unused_must_use and deprecated without mass_cfg_attr
    Union { f: 0.0 };
}
