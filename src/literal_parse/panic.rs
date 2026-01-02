#[doc(hidden)]
#[macro_export]
macro_rules! concat_panic {
    ($($msg: expr),+ $(,)?) => {
        {
            let buf = $crate::literal_parse::concat_strs::<{ 0 $(+ $msg.as_bytes().len())+ }>(&[$($msg),+]);
            let msg = unsafe { core::str::from_utf8_unchecked(&buf) }; // SAFETY: in concat_strs, we concatenated the strings byte slices directly, so the result is valid UTF-8
            panic!("{}", msg);
        }
    };
}

#[doc(hidden)]
pub const fn concat_strs<'a, const LEN: usize>(msgs: &[&'a str]) -> [u8; LEN] {
    let mut i = 0;
    let mut write_index = 0;
    let mut buf: [u8; LEN] = [0; LEN];

    while i < msgs.len() {
        let msg_bytes = msgs[i].as_bytes();

        let mut j = 0;
        while j < msg_bytes.len() {
            buf[write_index] = msg_bytes[j];
            j += 1;
            write_index += 1;
        }
        i += 1;
    }
    assert!(write_index == LEN); // should have correctly determined LEN from the macro which called this function
    buf
}