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
