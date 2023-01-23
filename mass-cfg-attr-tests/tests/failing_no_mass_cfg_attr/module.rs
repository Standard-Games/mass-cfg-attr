#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

#[deprecated]
mod test {
    pub fn ok() {}
}

fn main() {
    // Violates unused_must_use and deprecated without mass_cfg_attr
   test::ok();
}
