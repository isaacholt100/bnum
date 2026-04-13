/// TODO: write docs for this
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
