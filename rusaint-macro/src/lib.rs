extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Element, attributes(lsdata, lsevents, ignore))]
pub fn derive_element(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    TokenStream::new()
}
