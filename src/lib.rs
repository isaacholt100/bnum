//#![feature(const_generics)]
//#![feature(const_evaluatable_checked)]

mod uint;
mod iint;
mod tryops;
mod sign;
mod main;
mod arch;

pub use iint::BIint;
pub use sign::Sign;
pub use uint::BUint;

pub type U256 = BUint::<4>;
pub type U512 = BUint::<8>;
pub type U1024 = BUint::<16>;
pub type U2048 = BUint::<32>;
pub type U4096 = BUint::<64>;
pub type U8192 = BUint::<128>;

pub type ParseIntError = std::num::ParseIntError;
pub type TryFromIntError = &'static str;