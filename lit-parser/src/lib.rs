extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, LitInt};
use quote::quote;
use core::str::FromStr;
use proc_macro_error::{proc_macro_error, Diagnostic, Level};

fn get_bitwidth_signed(suffix: &str) -> Result<(usize, bool), ()> {
    let first = suffix.as_bytes().get(0);
    match first {
        Some(b'u' | b'i') => {
            let bit_width = usize::from_str(&suffix[1..]).map_err(|_e| ())?;
            Ok((bit_width, first == Some(&b'i')))
        },
        _ => Err(()),
    }
}

// convert base10 to base256 digits
fn base10_to_u8_digits(src: &str) -> Vec<u8> {
    let cap = src.len() / 2; // log_256 (10) ~ 0.415
    let mut digits = Vec::with_capacity(cap);
    let mut current = 0u16;
    let mut current_base = 1;
    for c in src.chars().rev() {
        let base10_digit = c.to_digit(10).unwrap();
        current += base10_digit as u16 * current_base;
        current_base *= 10;
        if current >= 256 {
            // digits.push((current / 256) as u8);
        }
    }
    digits

}

fn base10_to_base256(src: &str) -> Vec<u8> {
    let mut digits = src.chars().map(|c| c.to_digit(10).expect("invalid digit") as u8).collect::<Vec<u8>>();
    let mut result = Vec::new();

    while !digits.is_empty() {
        let mut remainder = 0u16;
        let mut new_digits = Vec::new();

        for &digit in &digits {
            let value = remainder * 10 + digit as u16;
            let quotient = value / 256;
            remainder = value % 256;
            if !new_digits.is_empty() || quotient != 0 {
                new_digits.push(quotient as u8);
            }
        }

        result.push(remainder as u8);
        digits = new_digits;
    }

    result.reverse(); // Most significant digit first
    if result.is_empty() {
        vec![0]
    } else {
        result
    }
}

#[proc_macro]
#[proc_macro_error]
pub fn n(input: TokenStream) -> TokenStream {
    println!("{}", input.to_string());
    let literal = parse_macro_input!(input as LitInt);
    //u(input)
    let suffix = literal.suffix();
    match get_bitwidth_signed(suffix) {
        Ok((bit_width, signed)) => {
            // literal.base10_parse();
            dbg!(bit_width, suffix);
            let digits = literal.base10_digits();
            println!("{} {:?}", bit_width, signed);
            dbg!(digits);
            if bit_width % 8 != 0 {
                let message = format!("invalid width `{}` for number literal", bit_width);
                Diagnostic::spanned(literal.span(), Level::Error, message)
                    .help("the width must be a multiple of 8".into())
                    .emit();
                return quote!{0}.into()
            }
            if digits.starts_with('-') {
                let message = format!("cannot apply unary operator `-` to type `{}`", suffix.to_ascii_uppercase());
                Diagnostic::spanned(literal.span(), Level::Error, message)
                    .help("unsigned values cannot be negated".into())
                    .emit();
                return quote!{0}.into()
            }
            let byte_width = bit_width / 8;
            let bytes = base10_to_base256(digits);
            if bytes.len() > byte_width {
                let message = format!("literal out of range for `{}`", suffix.to_ascii_uppercase());
                Diagnostic::spanned(literal.span(), Level::Error, message)
                    .help(format!("the literal `{}` does not fit into the type `{}`", literal, suffix.to_ascii_uppercase()))
                    .emit();
                return quote!{0}.into()
            }
            let ty = quote::format_ident!("{}", if signed { "Int" } else { "Uint" });
            let output = quote! {
                const {
                    bnum::#ty::<#byte_width>::from_le_slice(&[#(#bytes),*]).unwrap()
                }
            };
            output.into()
        },
        Err(()) => {
            let message = format!("invalid suffix `{}` for number literal", suffix);
            Diagnostic::spanned(literal.span(), Level::Error, message)
                .help("the suffix must be of the form {u,i}N, where N is the bit width (e.g. `u256, i512`, etc.)".into())
                .emit();
            quote!{0}.into() // this avoid the "expected expression, found end of macro arguments" error
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