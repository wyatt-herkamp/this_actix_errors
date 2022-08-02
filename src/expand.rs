use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::collections::BTreeSet as Set;
use syn::spanned::Spanned;
use syn::{
    Data, DeriveInput, GenericArgument, Member, PathArguments, Result, Token, Type, Visibility,
};

pub fn execute(node: DeriveInput) -> Result<TokenStream> {
    let span = node.span();

    let attrs = node.attrs;
    match node.data {
        Data::Enum(data) => {
            let ident = node.ident;
            super::expand_enum::execute(ident, data, attrs)
        }
        v => {
            Err(syn::Error::new(
                span,
                "Only enums can be derived",
            ))
        }
    }
}