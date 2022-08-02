mod expand_enum;
mod expand;
mod attrs;

use proc_macro::TokenStream;
use proc_macro2::Ident;

use syn::{parse_macro_input, DeriveInput};
use quote::quote;


#[proc_macro_derive(ActixError, attributes(message, status_code))]
pub fn actix_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand::execute(input).unwrap_or_else(|e| e.to_compile_error())
        .into()
}
