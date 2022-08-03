use proc_macro2::{Delimiter, Group, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use std::collections::BTreeSet as Set;
use std::iter::FromIterator;
use syn::parse::{Nothing, Parser, ParseStream};
use syn::{braced, bracketed, parenthesized, token, Attribute, Error, Ident, Index, LitInt, LitStr, Result, Token, ItemConst, ItemEnum};

pub enum StatusCode {
    StatusCodeAsInt(LitInt),
    StatusCode(Ident),
}

pub struct Attributes {
    pub message: Option<LitStr>,
    pub status_code: Option<StatusCode>,
}

impl Attributes {
    pub fn get(input: &[Attribute]) -> Result<Self> {
        let mut attrs = Attributes {
            message: None,
            status_code: None,
        };
        for attr in input {
            if attr.path.is_ident("message") {
                attr.parse_args_with(|input: ParseStream| {
                    attrs.message = Some(input.parse()?);
                    Ok(())
                })?;
            } else if attr.path.is_ident("status_code") {
                attr.parse_args_with(|input: ParseStream| {
                    if let Ok(v) = input.parse::<LitInt>() {
                        attrs.status_code = Some(StatusCode::StatusCodeAsInt(v));
                    }else{
                        attrs.status_code = Some(StatusCode::StatusCode(input.parse()?));
                    }
                    Ok(())
                })?;
            }
        }
        Ok(attrs)
    }
}

