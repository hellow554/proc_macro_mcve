extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Dump)]
pub fn derive_dump(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    quote!(
        impl foo_dump::Dump for #name {
            fn dump<D: foo_dump::Dumper>(&self, d: D) {
                unimplemented!()
            }
        }
    ).into()
}
