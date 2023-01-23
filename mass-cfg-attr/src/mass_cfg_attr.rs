use crate::helpers::VenialResult;
use crate::inner_from_attr::inner_from_attr;
use crate::split_mass_cfg_attr;
use crate::split_mass_cfg_attr::MassCfgAttrParts;
use proc_macro2::{Delimiter, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::ToTokens;
use std::str::FromStr;
use venial::{Attribute, AttributeValue, Declaration, GroupSpan, ImplMember, StructFields};

pub fn transform(inner: TokenStream, input: TokenStream) -> VenialResult<TokenStream> {
    let attr_parts = split_mass_cfg_attr::split_mass_cfg_attr_inner(inner)?;

    let mut declaration = venial::parse_declaration(input)?;
    decorate_declaration(&attr_parts, &mut declaration)?;
    Ok(declaration.to_token_stream())
}

fn decorate_declaration(
    attr_parts: &MassCfgAttrParts,
    declaration: &mut Declaration,
) -> Result<(), venial::Error> {
    let decorate = |attrs: &mut [Attribute]| decorate_attributes(attr_parts, attrs);
    decorate(declaration.attributes_mut());

    match declaration {
        Declaration::Struct(struct_declration) => match &mut struct_declration.fields {
            StructFields::Unit => {}
            StructFields::Tuple(tuple_fields) => tuple_fields
                .fields
                .inner
                .iter_mut()
                .map(|(field, _punct)| &mut field.attributes)
                .for_each(|attrs| decorate(attrs)),
            StructFields::Named(named_fields) => named_fields
                .fields
                .inner
                .iter_mut()
                .map(|(field, _punct)| &mut field.attributes)
                .for_each(|attrs| decorate(attrs)),
        },
        Declaration::Enum(enum_declaration) => enum_declaration
            .variants
            .inner
            .iter_mut()
            .map(|(field, _punct)| &mut field.attributes)
            .for_each(|attrs| decorate(attrs)),
        Declaration::Union(union_declaration) => union_declaration
            .fields
            .fields
            .inner
            .iter_mut()
            .map(|(field, _punct)| &mut field.attributes)
            .for_each(|attrs| decorate(attrs)),
        Declaration::Module(module_declaration) => {
            decorate(&mut module_declaration.attributes);
            decorate(&mut module_declaration.inner_attributes);
            // Note: we do not decorate the inner parts of a module... but we could
        }
        Declaration::Impl(impl_declaration) => {
            decorate(&mut impl_declaration.inner_attributes);
            impl_declaration
                .body_items
                .iter_mut()
                .try_for_each(|item| {
                    match item {
                        ImplMember::Method(method) => decorate(&mut method.attributes),
                        ImplMember::Constant(constant) => decorate(&mut constant.attributes),
                        ImplMember::AssocTy(associated_type) => {
                            decorate(&mut associated_type.attributes)
                        }
                        _ => {
                            return Err(venial::Error::new(
                                "Unsupported structure, can not apply mass_cfg_attr",
                            ));
                        }
                    }
                    Ok(())
                })?;
        }
        Declaration::TyDefinition(_type_definition_declaration) => {}
        Declaration::Function(_function_declation) => {}
        Declaration::Use(_use_declaration) => {}
        _ => {
            return Err(venial::Error::new(
                "Unsupported structure, can not apply mass_cfg_attr",
            ));
        }
    }
    Ok(())
}

/// Produces a `#[cfg_attr(..., ...)]` attribute that with the provided contents from
/// `#[mass_cfg_attr(...)]` for whatever attribute is provided
fn create_cfg_attr(predicate: &TokenStream, old_attr: &Attribute) -> Attribute {
    let tk_hash = Punct::new('#', Spacing::Joint);

    let tk_brackets = GroupSpan {
        delimiter: Delimiter::Bracket,
        span: Span::call_site(),
    };

    let path = vec![TokenTree::Ident(Ident::new("cfg_attr", Span::call_site()))];

    // ToDo: Make the value creation not awful
    let new_attribute_values = format!("{}, {}", predicate, inner_from_attr(old_attr));

    let ts = TokenStream::from_str(&new_attribute_values).unwrap();
    let tokens: Vec<_> = ts.into_iter().collect();
    let value = AttributeValue::Group(
        GroupSpan {
            delimiter: Delimiter::Parenthesis,
            span: Span::call_site(),
        },
        tokens,
    );

    Attribute {
        tk_hash,
        tk_bang: None,
        tk_brackets,
        path,
        value,
    }
}

fn decorate_attributes(mass_cfg_attr_parts: &MassCfgAttrParts, attrs: &mut [Attribute]) {
    for attr in attrs {
        if let Some(path) = attr.get_single_path_segment() {
            if mass_cfg_attr_parts.affected_attrs.contains(path) {
                let new_attr = create_cfg_attr(&mass_cfg_attr_parts.predicate, attr);
                let _old_attr = std::mem::replace(attr, new_attr);
            }
        }
    }
}
