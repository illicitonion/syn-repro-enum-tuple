extern crate proc_macro;
extern crate syn;

use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UsingSyn)]
pub fn derive_into_primitive(stream: TokenStream) -> TokenStream {
    parse_macro_input!(stream as DeriveInput);
    "".parse().unwrap()
}
