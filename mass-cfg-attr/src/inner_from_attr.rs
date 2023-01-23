use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::ToTokens;
use venial::Attribute;

pub fn inner_from_attr(attr: &Attribute) -> TokenStream {
    let Some(group) = attr.to_token_stream().into_iter().filter_map(|tt| match tt {
        TokenTree::Group(group) => Some(group),
        _ => None,
    }).next() else {
        panic!("This should not be possible");
    };
    group.stream()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use std::str::FromStr;
    use venial::Declaration;

    fn get_attr_str(attr_stt: &str) -> Attribute {
        let ts = TokenStream::from_str(&format!("{}\nfn fun(){}", attr_stt, "{}")).unwrap();
        let Ok(Declaration::Function(fun)) = venial::parse_declaration(ts) else {
            panic!("This should not be possible")
        };
        fun.attributes[0].clone()
    }

    #[test]
    fn test_inner_from_attr() {
        let attr = get_attr_str("#[simple]");
        assert_eq!(&inner_from_attr(&attr).to_string(), "simple");

        let attr = get_attr_str("#[grouped(value)]");
        assert_eq!(&inner_from_attr(&attr).to_string(), "grouped (value)"); // unclear why changed

        let attr = get_attr_str("#[key = value]");
        assert_eq!(&inner_from_attr(&attr).to_string(), "key = value");

        let attr = get_attr_str("#[grouped(key = value)]");
        assert_eq!(&inner_from_attr(&attr).to_string(), "grouped (key = value)");
        // as above
    }
}
