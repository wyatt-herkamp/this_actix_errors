use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Ident, LitInt, LitStr, Result,
};
mod keywords {
    syn::custom_keyword!(message);
    syn::custom_keyword!(status_code);
    syn::custom_keyword!(tracing_header);
}
#[derive(Clone, Debug)]
pub enum StatusCode {
    StatusCodeAsInt(LitInt),
    StatusCode(Ident),
}
impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::StatusCode(Ident::new(
            "INTERNAL_SERVER_ERROR",
            proc_macro2::Span::call_site(),
        ))
    }
}
impl ToTokens for StatusCode {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let status_code = match self {
            StatusCode::StatusCodeAsInt(v) => {
                quote! {
                    _this_actix_error::private::StatusCode::from_u16(#v).unwrap()
                }
            }
            StatusCode::StatusCode(v) => {
                quote! {
                    _this_actix_error::private::StatusCode::#v
                }
            }
        };

        status_code.to_tokens(tokens);
    }
}
impl Parse for StatusCode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitInt) {
            let lit: LitInt = input.parse()?;

            match lit.base10_parse::<u16>() {
                Ok(v) => {
                    // status code must be between 100 and 999 and not 0
                    if v < 100 || v > 999 && v != 0 {
                        return Err(syn::Error::new(
                            lit.span(),
                            "status code must be between 100 and 999",
                        ));
                    }
                }
                Err(_) => return Err(syn::Error::new(lit.span(), "status code must be a number")),
            }

            Ok(StatusCode::StatusCodeAsInt(lit))
        } else if input.peek(Ident) {
            let ident = input.parse()?;
            Ok(StatusCode::StatusCode(ident))
        } else {
            Err(input.error("expected status code"))
        }
    }
}
#[derive(Default, Clone)]
pub struct Attributes {
    pub message: Option<LitStr>,
    pub status_code: Option<StatusCode>,
}

impl Attributes {
    pub fn get(input: &[Attribute]) -> Result<Self> {
        let mut attrs = Attributes::default();
        for attr in input {
            if attr.path().is_ident("message") {
                let message = attr.parse_args()?;
                attrs.message = Some(message);
            } else if attr.path().is_ident("status_code") {
                let status_code = attr.parse_args()?;
                attrs.status_code = Some(status_code);
            }
        }
        Ok(attrs)
    }
}
#[derive(Default, Clone)]
pub struct ContainerAttr {
    /// Default status code
    pub status_code: StatusCode,
    pub trace_header: bool,
}

impl Parse for ContainerAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut container_attr = ContainerAttr::default();
        while !input.is_empty() {
            if input.peek(syn::Token![,]) {
                let _: syn::Token![,] = input.parse()?;
            }
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::status_code) {
                let _ = input.parse::<keywords::status_code>()?;
                let _: syn::Token![=] = input.parse()?;
                container_attr.status_code = input.parse()?;
            } else if lookahead.peek(keywords::tracing_header) {
                let _ = input.parse::<keywords::tracing_header>()?;
                container_attr.trace_header = true;
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(container_attr)
    }
}
