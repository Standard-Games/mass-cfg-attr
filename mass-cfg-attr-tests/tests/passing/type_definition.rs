#![deny(unused_must_use, deprecated)]
#![allow(path_statements)] // disable warning for test

use mass_cfg_attr::mass_cfg_attr;

#[mass_cfg_attr(test, deprecated)]
#[deprecated]
type TypeDefinition = ();

fn main() {
    // Violation of unused_must_use if not removed by mass_cfg_attr
    let _: TypeDefinition = ();
}
