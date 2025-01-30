// SPDX-FileCopyrightText: OpenTalk GmbH <mail@opentalk.eu>
//
// SPDX-License-Identifier: EUPL-1.2

//! Macros for opentalk-types-common.

#![deny(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    overflowing_literals,
    patterns_in_fns_without_body,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    //unused,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use opentalk_types_common_identifiers::module_id::ModuleId;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Create a constant `ModuleId` at compile time.
#[proc_macro]
pub fn module_id(input: TokenStream) -> TokenStream {
    let Ok(crate_name) = crate_name("opentalk-types-common") else {
        return quote! { compile_error!("Couldn't find opentalk-types-common crate") }.into();
    };

    let crate_name = match crate_name {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = proc_macro2::Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    };

    let input = parse_macro_input!(input as LitStr);

    let value = input.value();

    match value.parse::<ModuleId>() {
        Ok(value) => {
            let value = value.as_str();
            quote! {
                #crate_name::modules::ModuleId::__new_borrowed(#value)
            }
        }
        Err(e) => {
            let msg = e.to_string();
            quote! { compile_error!(#msg) }
        }
    }
    .into()
}
