extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input};

#[proc_macro]
pub fn test_proc(input: TokenStream) -> TokenStream {
    println!("{:?}", input);
    //input
    "ONE".parse().unwrap()
}