//! Derive macros for the `etheryal-extension` crate.
#![deny(missing_docs)]
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(extension_message))]
/// Sets the destination of an extension message.
struct MacroArgs {
    /// A message that is sent from the host to the guest.
    guest: Option<()>,

    /// A message that is sent from the guest to the host.
    host: Option<()>,
}

/// Derives the `ExtensionMessage` trait for the given type.
#[proc_macro_derive(ExtensionMessage, attributes(extension_message))]
pub fn derive_extension_message(input: TokenStream) -> TokenStream {
    let etheryal_extension = match crate_name("etheryal-extension") {
        Ok(found_crate) => {
            let etheryal_extension = match found_crate {
                FoundCrate::Itself => Ident::new("etheryal_extension", Span::call_site()),
                FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
            };
            quote! { #etheryal_extension }
        },
        _ => {
            let found_crate = crate_name("etheryal-extension-common")
                .expect("etheryal-extension-common crate should be present");
            let etheryal_extension = match found_crate {
                FoundCrate::Itself => Ident::new("crate", Span::call_site()),
                FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
            };
            quote! { #etheryal_extension::message }
        },
    };

    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let attr: MacroArgs = FromDeriveInput::from_derive_input(&ast).expect("attribute parsed");

    let mut tokens = proc_macro2::TokenStream::new();
    if attr.guest.is_some() {
        tokens.extend(quote! {
            impl #etheryal_extension::GuestMessage for #name {}
        });
    }
    if attr.host.is_some() {
        tokens.extend(quote! {
            impl #etheryal_extension::HostMessage for #name {}
        });
    }
    tokens.into()
}
