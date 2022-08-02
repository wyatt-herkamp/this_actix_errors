use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use std::collections::BTreeSet as Set;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DataEnum, DeriveInput, Fields, GenericArgument, Member, PathArguments, Result, Token, Type, Visibility};
use crate::attrs;

pub fn execute(ident: Ident, node: DataEnum, attrs: Vec<Attribute>) -> Result<TokenStream> {
    let mut members = Vec::new();
    let mut status_codes = Vec::new();
    for arm in node.variants {
        let arm_ident = arm.ident;
        let v = attrs::Attributes::get(&arm.attrs)?;
        let  field_pat = match arm.fields {
            Fields::Named(_) => {
                quote! {
                    {..}
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    (_)
                }
            }
            Fields::Unit => {
                quote! {
                }
            }
        };
        let status_code = if let Some(status_code) = v.status_code {
            let v = quote! {
                  actix_web::http::StatusCode::from_u16(#status_code).unwrap()
            };
            status_codes.push(quote! {
                #ident::#arm_ident #field_pat => #v,
            });
            v
        } else {
            quote! {
                  actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        if let Some(message) = v.message {
            let response = quote! {
                #ident::#arm_ident #field_pat => {
                    (actix_web::HttpResponse::new(#status_code),actix_web::web::Bytes::from(#message))
                }
            };
            members.push(response);
        };
    }

    Ok(quote! {
        impl actix_web::ResponseError for #ident {
            fn status_code(&self) ->  actix_web::http::StatusCode {
                match self {
                    #(
                        #status_codes
                    ),*
                    _ =>  actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
            fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody>  {
                let (mut response, buf) = match self {
                    #(
                        #members
                    ),*
                    _ => {
                        (actix_web::HttpResponse::InternalServerError().finish(),actix_web::web::Bytes::from("Internal Server Error"))
                    },
                };
                response.headers_mut().insert(actix_web::http::header::CONTENT_TYPE, "text/plain; charset=utf-8".parse().unwrap());

                response.set_body(actix_web::body::BoxBody::new(buf))
            }
        }
    })
}