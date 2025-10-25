macro_rules! must_use_op {
    () => {
        "this returns the result of the operation, without modifying the original"
    };
    (float) => {
        "method returns a new number and does not mutate the original value"
    };
    (comparison) => {
        "this returns the result of the comparison, without modifying either input"
    };
}

pub(crate) use must_use_op;


macro_rules! default {
    () => {
        "Returns the default value of `Self::ZERO`."
    };
}

pub(crate) use default;
