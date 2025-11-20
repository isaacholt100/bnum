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

#[proc_macro]
#[proc_macro_error]
pub fn n(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitInt);
    let suffix = literal.suffix();
    match get_bitwidth_signed(suffix) {
        Ok((bit_width, signed)) => {
            let digits = literal.base10_digits();
            if bit_width % 8 != 0 {
                let message = format!("invalid width `{}` for number literal", bit_width);
                Diagnostic::spanned(literal.span(), Level::Error, message)
                    .help("the width must be a multiple of 8".into())
                    .emit();
                return quote!{0}.into()
            }
            let type_name = suffix.to_ascii_uppercase();
            
            if digits.starts_with('-') && !signed {
                let message = format!("cannot apply unary operator `-` to type `{}`", type_name);
                Diagnostic::spanned(literal.span(), Level::Error, message)
                    .help("unsigned values cannot be negated".into())
                    .emit();
                return quote!{0}.into()
            }
            let byte_width = bit_width / 8;
            // TODO: parse the digits into a byte array at compile time so we can display a compile error instead of panic, if the literal is out of range
            // let bytes = base10_to_base256(digits);
            // if bytes.len() > byte_width {
            //     let message = format!("literal out of range for `{}`", suffix.to_ascii_uppercase());
            //     Diagnostic::spanned(literal.span(), Level::Error, message)
            //         .help(format!("the literal `{}` does not fit into the type `{}`", literal, suffix.to_ascii_uppercase()))
            //         .emit();
            //     return quote!{0}.into()
            // }
            let ty = quote::format_ident!("{}", if signed { "Int" } else { "Uint" });
            let output = quote! {
                const {
                    match bnum::#ty::<#byte_width>::from_str_radix(#digits, 10) {
                        Ok(value) => value,
                        Err(_) => panic!(concat!("literal out of range for type ", #type_name))
                    }
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