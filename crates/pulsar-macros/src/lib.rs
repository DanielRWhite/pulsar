extern crate proc_macro;
use proc_macro::TokenStream;
//use syn::{ DeriveInput, parse_macro_input };
//use quote::quote;

#[proc_macro_attribute]
pub fn pulsar(args: TokenStream, input: TokenStream) -> TokenStream {
        println!("{}", &args);
        println!("{}", &input);
        input
}