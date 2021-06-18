use core::cmp::{PartialEq, PartialOrd, Ordering};
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub enum Float {
    NAN,
    INFINITY { positive: bool },
    Value {
        digits: Vec<f64>,
        positive: bool,
        exponent: f64,
    },
}

use Float::{NAN, INFINITY, Value};

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        match self {
            NAN => false,
            INFINITY { positive } => match other {
                INFINITY { positive: p } => {
                    p == positive
                },
                _ => false,
            },
            Value { digits, positive, exponent } => {
                match other {
                    Value { digits: d, positive: p, exponent: e } => {
                        positive == p && exponent == e && digits == d
                    },
                    _ => false,
                }
            },
        }
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (NAN, _) => None,
            (_, NAN) => None,
            (INFINITY { positive: true }, other) => Some({
                match other {
                    INFINITY { positive: true } => Ordering::Equal,
                    _ => Ordering::Greater,
                }
            }),
            (INFINITY { positive: false }, other) => Some({
                match other {
                    INFINITY { positive: false } => Ordering::Equal,
                    _ => Ordering::Less,
                }
            }),
            (s, INFINITY { positive: true }) => Some({
                match s {
                    INFINITY { positive: true } => Ordering::Equal,
                    _ => Ordering::Less,
                }
            }),
            (s, INFINITY { positive: false }) => Some({
                match s {
                    INFINITY { positive: false } => Ordering::Equal,
                    _ => Ordering::Greater,
                }
            }),
            (Value { digits, exponent, positive }, Value { digits: d, exponent: e, positive: p }) => {
                todo!()
            },
        }
    }
}

pub enum Rounding {
    Up
}

impl Float {
    const MAX_E: f64 = 3000.0;
    const MIN_E: f64 = -3000.0;
    const LOG_BASE: f64 = 7.0;
    const BASE: f64 = 1e7;
    pub fn finalise(&mut self, sd: Option<f64>, rounding: Rounding, truncated: bool, external: bool) {
        if let Value { digits, positive, exponent } = self {
            match sd {
                Some(sd) => {
                    let mut d = 1.0;
                    let mut k = digits[0];
                    while k >= 10.0 {
                        d += 1.0;
                        k /= 10.0;
                    }
                    let mut i = sd - d;
                    let mut j = 0.0;
                    let mut w = 0.0;
                    let mut xdi = 0.0;
                    let mut rd = 0.0;
                    if i < 0.0 {
                        i += Self::LOG_BASE;
                        j = sd;
                        w = digits[0];
                        rd = (w / f64::powf(10.0, d - j - 1.0) % 10.0).trunc();
                    } else {
                        xdi = ((i + 1.0) / Self::LOG_BASE).ceil();
                        k = digits.len() as f64;
                        if xdi >= k {
                            if truncated {
                                while k <= xdi {
                                    digits.push(0.0);
                                    k += 1.0;
                                }
                                w = 0.0;
                                rd = 0.0;
                                d = 1.0;
                                i %= Self::LOG_BASE;
                                j = i - Self::LOG_BASE + 1.0;
                            } else {
                                if external {
                                    if *exponent > Self::MAX_E {
                                        *self = INFINITY { positive: *positive };
                                    } else if *exponent < Self::MIN_E {
                                        *exponent = 0.0;
                                        digits.clear();
                                        digits.push(0.0);
                                    }
                                }
                                return;
                            }
                        } else {
                            w = digits[xdi as usize];
                            k = w;
                            d = 1.0;
                            while k >= 10.0 {
                                d += 1.0;
                                k /= 10.0;
                            }
                            i %= Self::LOG_BASE;
                            j = i - Self::LOG_BASE + d;
                            rd = if j < 0.0 {
                                0.0
                            } else {
                                (w / f64::powf(10.0, d - j - 1.0) % 10.0).trunc()
                            };
                        }
                    }
                    let truncated = truncated || sd.is_sign_negative() || digits.get(xdi as usize + 1) != None || (if j.is_sign_negative() {
                        w
                    } else {
                        w % f64::powf(10.0, d - j - 1.0)
                    }) != 0.0;
                    let round_up = true;
                    if sd < 1.0 || digits[0] == 0.0 {
                        digits.clear();
                        if round_up {
                            let mut sd = sd;
                            sd -= *exponent + 1.0;
                            digits.push(f64::powf(10.0, (Self::LOG_BASE - sd % Self::LOG_BASE) % Self::LOG_BASE));
                            *exponent = -sd.trunc();
                        } else {
                            digits.push(0.0);
                            *exponent = 0.0;
                        }
                        return;
                    }
                    if i == 0.0 {
                        println!("trunc 1");
                        digits.truncate(xdi as usize);
                        k = 1.0;
                        xdi -= 1.0;
                    } else {
                        k = f64::powf(10.0, Self::LOG_BASE - i);
                        println!("trunc 2");
                        digits.truncate(xdi as usize);
                        digits.push(if j.is_sign_positive() {
                            (w / f64::powf(10.0, d - j) % f64::powf(10.0, j)).trunc() * k
                        } else {
                            0.0
                        });
                    }
                    if round_up {
                        loop {
                            if xdi == 0.0 {
                                i = 1.0;
                                j = digits[0];
                                while j >= 10.0 {
                                    i += 1.0;
                                    j /= 10.0;
                                }
                                j = k;
                                digits[0] += k;
                                k = 1.0;
                                while j >= 10.0 {
                                    k += 1.0;
                                    j /= 10.0;
                                }
                                if i != k {
                                    *exponent += 1.0;
                                    if digits[0] == Self::BASE {
                                        digits[0] = 1.0;
                                    }
                                }
                                break;
                            } else {
                                digits[xdi as usize] += k;
                                if digits[xdi as usize] != Self::BASE {
                                    break;
                                }
                                digits[xdi as usize] = 0.0;
                                xdi -= 1.0;
                                k = 1.0;
                            }
                        }
                    }
                    let mut index = digits.len();
                    while digits[index - 1] == 0.0 {
                        println!("popping");
                        digits.pop();
                        index -= 1;
                    }
                    if external {
                        if *exponent > Self::MAX_E {
                            *self = INFINITY { positive: *positive };
                        } else if *exponent < Self::MIN_E {
                            *exponent = 0.0;
                            digits.clear();
                            digits.push(0.0);
                        }
                    }
                },
                None => {}
            }
        }
    }
    pub fn abs(&self) -> Self {
        match self {
            NAN => NAN,
            INFINITY { positive: _ } => INFINITY { positive: true },
            Value { digits, exponent, positive: _ } => {
                Value {
                    digits: digits.clone(),
                    exponent: *exponent,
                    positive: true,
                }
            }
        }
    }
}