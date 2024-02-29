use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DataEnum, Result};

use crate::attrs::{self, ContainerAttr};
pub fn execute(
    ident: Ident,
    data_enum: DataEnum,
    container_attrs: Vec<Attribute>,
) -> Result<TokenStream> {
    let mut container_attr: ContainerAttr = container_attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("response_error"))
        .map(|v| v.parse_args::<ContainerAttr>())
        .transpose()?
        .unwrap_or_default();
    let mut status_codes = Vec::with_capacity(data_enum.variants.len());
    let mut members = Vec::with_capacity(data_enum.variants.len());
    for item in data_enum.variants.iter() {
        let arm_ident = &item.ident;
        let attr = attrs::Attributes::get(&item.attrs)?;
        let status_code = attr
            .status_code
            .unwrap_or_else(|| container_attr.status_code.clone());
        status_codes.push(quote! {
            #ident::#arm_ident { .. } => #status_code,
        });
        let response = if let Some(message) = attr.message {
            quote! {
                #ident::#arm_ident { .. } => {
                    (_this_actix_error::private::HttpResponse::new(#status_code), _this_actix_error::private::Bytes::from(#message))
                }
            }
        } else {
            quote! {
                #ident::#arm_ident { .. } => {
                    (_this_actix_error::private::HttpResponse::new(#status_code), _this_actix_error::private::Bytes::from(format!("{}",self)))
                }
            }
        };
        members.push(response);
    }

    let function_headers = if container_attr.trace_header {
        #[cfg(feature = "tracing")]
        quote! {
            #[_this_actix_error::private::instrument(self)]
        }
        #[cfg(not(feature = "tracing"))]
        quote! {}
    } else {
        quote! {}
    };
    let result = quote! {
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate this_actix_error as _this_actix_error;
            impl _this_actix_error::private::ResponseError for #ident {
                fn status_code(&self) ->  _this_actix_error::private::StatusCode {
                    match self {
                        #(
                            #status_codes
                        )*
                        _ =>  _this_actix_error::private::StatusCode::INTERNAL_SERVER_ERROR,
                    }
                }
                #function_headers
                fn error_response(&self) -> _this_actix_error::private::ActixErrorResponse  {
                    let (mut response, buf) = match self {
                        #(
                            #members
                        )*
                        _ => {
                            (_this_actix_error::private::HttpResponse::InternalServerError().finish(), _this_actix_error::private::Bytes::from(format!("{}", self)))
                        },
                    };
                    response.headers_mut().insert(_this_actix_error::private::CONTENT_TYPE, _this_actix_error::private::CONTENT_TYPE_VALUE.clone());
                    response.set_body(_this_actix_error::private::BoxBody::new(buf))
                }
            }
        };
    };
    Ok(result)
}
