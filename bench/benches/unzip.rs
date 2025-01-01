macro_rules! unzip {
    (fn $name: ident <$($Gen: ident), *>) => {
        paste::paste! {
            pub fn $name<$($Gen), *, I>(i: I) -> ($(Vec<$Gen>), *)
            where I: Iterator<Item = ($($Gen), *)>
            {
                let ($(mut [<vec_ $Gen:lower>]), *) = match i.size_hint().1 {
                    Some(size) => ($(Vec::<$Gen>::with_capacity(size)), *),
                    None => ($(Vec::<$Gen>::new()), *),
                };
                i.for_each(|($([<$Gen:lower>]), *)| {
                    $(
                        [<vec_ $Gen:lower>].push([<$Gen:lower>]);
                    )*
                });
                ($([<vec_ $Gen:lower>]), *)
            }
        }
    };
}

// unzip!(fn unzip3<T1, T2, T3>);
unzip!(fn unzip2<T1, T2>);