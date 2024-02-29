mod attrs;
mod expand_enum;

use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

#[proc_macro_derive(ActixError, attributes(message, status_code))]
pub fn actix_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let span = input.span();

    let attrs = input.attrs;
    let result = match input.data {
        Data::Enum(data) => {
            let ident = input.ident;
            expand_enum::execute(ident, data, attrs)
        }
        _ => Err(syn::Error::new(span, "Only enums can be derived")),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
