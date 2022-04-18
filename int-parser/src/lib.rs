extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, LitInt};
use quote::quote;
//use bint::BUint;

#[proc_macro]
pub fn n(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitInt);
    //u(input)
    println!("{:?}", literal.suffix());
    println!("{:?}", literal.base10_digits());
    let base10_digits = literal.base10_digits();
    /*match literal.base10_parse::<>() {
        Ok(num) => {

        },
        Err(err) => {

        }
    }*/
    //let u = BUint::<8>::from_str(literal.base10_digits()).expect("");
    println!("{}", literal);
    let output = quote! {
        {
            panic!("integer too large");
        core::str::FromStr::from_str(#base10_digits).expect("integer literal is too large")
        }
    };
    output.into()
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