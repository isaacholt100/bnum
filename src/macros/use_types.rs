/// Declares a type alias from a type descriptor with the same name.
/// 
/// `use_types!` takes an arbitrary number of type descriptors, and creates a type alias for each one with the same name as the descriptor, using the [`t!`](crate::t) macro. Each descriptor may be preceded by the `pub` visibility modifier, which makes the corresponding type alias public.
/// 
/// See the documentation of the [`t!`](crate::t) macro for details on type descriptors.
/// 
/// # Examples
/// 
/// ```
/// mod my_types {
///     bnum::use_types!{U256, I512w, pub U257s}
///     // creates type aliases for the `U256`, `I512w`, and `U257s` types, with `U257s` being made public
/// }
/// 
/// assert_eq!(my_types::U257s::BITS, 257);
/// // can only access `U257s` outside of `my_types` module
/// 
/// ```
#[macro_export]
macro_rules! use_types {
    { $($vis: vis $Type: ident),* $(,)? } => {
        $(
            $vis type $Type = $crate::t!($Type);
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn cases_use_types_macro() {
        mod types {
            #[allow(unused)]
            use_types! {
                pub U256w,
                pub I512s,
                pub U123p,
                I257,
                U1024,
            }
        }
        use types::*;
        // check that the pub visibility modifier exported the right types
        assert_eq!(U256w::BITS, 256);
        assert_eq!(I512s::OVERFLOW_MODE, crate::OverflowMode::Saturate);
        assert!(!U123p::MIN.is_negative_internal());
    }
}
