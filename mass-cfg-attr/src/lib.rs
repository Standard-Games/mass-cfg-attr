mod group_to_ident;
mod helpers;
mod inner_from_attr;
mod mass_cfg_attr;
mod split_mass_cfg_attr;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mass_cfg_attr(meta: TokenStream, input: TokenStream) -> TokenStream {
    match mass_cfg_attr::transform(meta.into(), input.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
