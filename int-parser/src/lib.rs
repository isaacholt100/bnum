extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn n(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitInt);
    //u(input)
    println!("{:?}", literal.suffix());
    println!("{:?}", literal.base10_digits());
    /*match literal.base10_parse::<>() {
        Ok(num) => {

        },
        Err(err) => {

        }
    }*/
    println!("{}", literal);
    "1".parse().unwrap()
}

#[proc_macro]
pub fn u(input: TokenStream) -> TokenStream {
    //println!("{:?}", input);
    let mut iter = input.into_iter();
    let token = iter.next().expect("Expected one argument");
    assert!(iter.next().is_none(), "Expected one argument");
    match token {
        TokenTree::Literal(literal) => {
            let tt = TokenTree::from(literal);
            let ts = TokenStream::from(tt);
            let _literal = parse_macro_input!(ts as LitInt);
            println!("");
        },
        _ => panic!("Expected number literal")
    }
    "1".parse().unwrap()
}