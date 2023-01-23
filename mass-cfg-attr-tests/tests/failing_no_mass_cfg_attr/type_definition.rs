#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

#[deprecated]
type TypeDefinition = ();

fn main() {
    // Violation of unused_must_use if not removed by mass_cfg_attr
    let _: TypeDefinition = ();
}
