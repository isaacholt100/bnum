macro_rules! const_fn {
    { $(#[$attr: meta]) * $vis: vis const $($rest: tt) + } => {
        #[cfg(feature = "nightly")]
        $(#[$attr]) *
        #[doc = "\n\nNB: this method is only `const` when the `nightly` feature is enabled."]
        $vis const $($rest) +

        #[cfg(not(feature = "nightly"))]
        $(#[$attr]) *
        $vis $($rest) +
    };
}

pub(crate) use const_fn;

macro_rules! const_fns {
    { $($(#[$attr: meta]) * $vis: vis const fn $name: ident ($($args: tt) *) -> $ret : ty { $($f: tt) + }) * } => {
        $(
            crate::nightly::const_fn! {
                $(#[$attr]) * $vis const fn $name ($($args) *) -> $ret { $($f) + }
            }
        )*
    };
    { $($(#[$attr: meta]) * $vis: vis const unsafe fn $name: ident ($($args: tt) *) -> $ret : ty { $($f: tt) + }) * } => {
        $(
            crate::nightly::const_fn! {
                $(#[$attr]) * $vis const unsafe fn $name ($($args) *) -> $ret { $($f) + }
            }
        )*
    };
}

pub(crate) use const_fns;

macro_rules! option_try {
    ($e: expr) => {
        match $e {
            Some(v) => v,
            None => return None,
        }
    };
}

pub(crate) use option_try;

macro_rules! ok {
    { $e: expr } => {
        match $e {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    };
}

pub(crate) use ok;
