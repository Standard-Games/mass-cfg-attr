use proc_macro2::{Group, Ident, TokenTree};

#[derive(Debug, PartialEq)]
pub(crate) enum GroupToIdentsError {
    ContainedGroup,
    ContainedLiteral,
    UnexpectedPunctuation(char),
}

impl From<GroupToIdentsError> for venial::Error {
    fn from(value: GroupToIdentsError) -> Self {
        match value {
            GroupToIdentsError::ContainedGroup => {
                Self::new("this group may not contain sub groups")
            }
            GroupToIdentsError::ContainedLiteral => {
                Self::new("this group may not contain literals")
            }
            GroupToIdentsError::UnexpectedPunctuation(c) => {
                Self::new(format!("found unexpected punctuation '{}' in group", c))
            }
        }
    }
}

pub(crate) fn group_to_idents(group: &Group) -> Result<Vec<Ident>, GroupToIdentsError> {
    let mut output = Vec::new(); // ToDo: Would be nice to calculate the size ahead of time
    let mut tokens = group.stream().into_iter().peekable();
    while let Some(tt) = tokens.next() {
        match tt {
            TokenTree::Ident(ident) => output.push(ident),
            TokenTree::Punct(p) => Err(GroupToIdentsError::UnexpectedPunctuation(p.as_char()))?,
            TokenTree::Group(_) => Err(GroupToIdentsError::ContainedGroup)?,
            TokenTree::Literal(_) => Err(GroupToIdentsError::ContainedLiteral)?,
        }
        if let Some(TokenTree::Punct(next_punct)) = tokens.peek() {
            // skip the next character if its a comma, otherwise the next loop will catch the error
            if next_punct.as_char() == ',' {
                tokens.next();
            }
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::{Delimiter, Span, TokenStream};
    use std::str::FromStr;

    #[test]
    fn test_group_to_idents() {
        let good_group = Group::new(
            Delimiter::Bracket,
            TokenStream::from_str("one, two, three").unwrap(),
        );
        let idents = group_to_idents(&good_group).unwrap();
        assert_eq!(
            idents,
            vec![
                Ident::new("one", Span::call_site()),
                Ident::new("two", Span::call_site()),
                Ident::new("three", Span::call_site()),
            ]
        )
    }

    #[test]
    fn test_group_in_group_to_idents() {
        let group_in_group = Group::new(
            Delimiter::Bracket,
            TokenStream::from_str("one, (two), three").unwrap(),
        );
        let error = group_to_idents(&group_in_group).unwrap_err();
        assert_eq!(error, GroupToIdentsError::ContainedGroup);
    }

    #[test]
    fn test_literal_in_group_to_idents() {
        let literal_in_group = Group::new(
            Delimiter::Bracket,
            TokenStream::from_str("one, 2, three").unwrap(),
        );
        let error = group_to_idents(&literal_in_group).unwrap_err();
        assert_eq!(error, GroupToIdentsError::ContainedLiteral);
    }

    #[test]
    fn test_dots_in_group_to_idents() {
        let dots = Group::new(
            Delimiter::Bracket,
            TokenStream::from_str("one. two. three").unwrap(),
        );
        let error = group_to_idents(&dots).unwrap_err();
        assert_eq!(error, GroupToIdentsError::UnexpectedPunctuation('.'));
    }

    #[test]
    fn test_missing_ident_in_group_to_idents() {
        let dots = Group::new(
            Delimiter::Bracket,
            TokenStream::from_str("one, , three").unwrap(),
        );
        let error = group_to_idents(&dots).unwrap_err();
        assert_eq!(error, GroupToIdentsError::UnexpectedPunctuation(','));
    }
}
