extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, LitInt};
use quote::quote;
use core::str::FromStr;
//use bint::BUint;

fn get_bitwidth_signed(suffix: &str) -> Result<(usize, bool), ()> {
    let first = suffix.as_bytes().get(0);
    match first {
        Some(b'u' | b'i') => {
            let bit_width = usize::from_str(&suffix[1..]).map_err(|_e| ())?;
            Ok((bit_width, first == Some(&b'u')))
        },
        _ => Err(()),
    }
}

#[proc_macro]
pub fn n(input: TokenStream) -> TokenStream {
    println!("{}", input.to_string());
    let literal = parse_macro_input!(input as LitInt);
    //u(input)
    let suffix = literal.suffix();
    match get_bitwidth_signed(suffix) {
        Ok((bit_width, unsigned)) => {
            let digits = literal.base10_digits();
            println!("{} {}", bit_width, unsigned);
            let (n, ty) = if bit_width % 64 == 0 {
                (bit_width / 64, if unsigned { "BUint" } else { "BInt" })
            } else if bit_width % 32 == 0 {
                (bit_width / 32, if unsigned { "BUintD32" } else { "BIntD32" })
            } else if bit_width % 16 == 0 {
                (bit_width / 16, if unsigned { "BUintD16" } else { "BIntD16" })
            } else if bit_width % 8 == 0 {
                (bit_width / 8, if unsigned { "BUintD8" } else { "BIntD8" })
            } else {
                let output = quote! {
                    compile_error!(concat!("invalid width `", #bit_width, "` for integer literal. the width must be a multiple of 8"))
                };
                return output.into();
            };
            let ty = quote::format_ident!("{}", ty);
            let output = quote! {
                {
                    const PARSED: bnum::#ty::<#n> = bnum::#ty::<#n>::parse_str_radix(#digits, 10);
                    PARSED
                }
            };
            output.into()
        },
        Err(()) => {
            let output = quote! {
                compile_error!(concat!("invalid suffix `", #suffix, "` for number literal"))
            };
            output.into()
        },
    }
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