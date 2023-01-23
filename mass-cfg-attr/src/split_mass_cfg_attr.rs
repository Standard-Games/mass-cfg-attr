use crate::group_to_ident;
use crate::group_to_ident::GroupToIdentsError;
use proc_macro2::{Ident, TokenStream, TokenTree};

pub(crate) struct MassCfgAttrParts {
    pub predicate: TokenStream,
    pub affected_attrs: Vec<Ident>,
}

fn find_comma(vec_tokens: &[TokenTree]) -> Option<usize> {
    vec_tokens.iter().position(|item| match item {
        TokenTree::Punct(punct) => punct.as_char() == ',',
        _ => false,
    })
}

#[derive(Debug)]
pub(crate) enum SplitInnerError {
    NotEnoughArguments,
    TooManyArguments,
    InvalidArguments,
    GroupToIdentsError(GroupToIdentsError),
}

impl From<SplitInnerError> for venial::Error {
    fn from(value: SplitInnerError) -> Self {
        match value {
            SplitInnerError::NotEnoughArguments => {
                venial::Error::new("Not enough arguments, mass_cfg_attr takes two arguments, a predicate and a list of attributes to decorate")
            }
            SplitInnerError::TooManyArguments => {
                venial::Error::new("Too many arguments, mass_cfg_attr takes two arguments, a predicate and a list of attributes to decorate")
            }
            SplitInnerError::InvalidArguments => {
                venial::Error::new("Invalid arguments found in mass_cfg_attr")
            }
            SplitInnerError::GroupToIdentsError(group_to_idents_error) => {
                group_to_idents_error.into()
            }
        }
    }
}

impl From<GroupToIdentsError> for SplitInnerError {
    fn from(value: GroupToIdentsError) -> Self {
        SplitInnerError::GroupToIdentsError(value)
    }
}

pub(crate) fn split_mass_cfg_attr_inner(
    inner: TokenStream,
) -> Result<MassCfgAttrParts, SplitInnerError> {
    let mut predicate: Vec<_> = inner.into_iter().collect();
    // Find the first comma
    let pos = find_comma(&predicate).ok_or(SplitInnerError::NotEnoughArguments)?;
    // split from the comma, then again to remove the comma
    let mut remainder = predicate.split_off(pos).split_off(1);
    // Check we have exactly one item left
    if remainder.len() != 1 {
        return Err(SplitInnerError::TooManyArguments);
    }
    // That item must be either an ident or a group, we want a Vec<Idents> so lets process it
    let affected_attrs = match remainder.remove(0) {
        TokenTree::Group(group) => group_to_ident::group_to_idents(&group)?,
        TokenTree::Ident(ident) => vec![ident],
        _ => return Err(SplitInnerError::InvalidArguments),
    };

    Ok(MassCfgAttrParts {
        predicate: predicate.into_iter().collect(),
        affected_attrs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::split_mass_cfg_attr::find_comma;
    use std::str::FromStr;

    #[test]
    fn test_find_comma() {
        let hello_world: Vec<_> = TokenStream::from_str("hello, world")
            .unwrap()
            .into_iter()
            .collect();
        assert_eq!(find_comma(&hello_world), Some(1));
        let hello: Vec<_> = TokenStream::from_str("hello")
            .unwrap()
            .into_iter()
            .collect();
        assert_eq!(find_comma(&hello), None);
    }
}
