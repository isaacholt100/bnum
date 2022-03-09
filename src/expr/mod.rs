use alloc::vec::Vec;
use alloc::boxed::Box;

trait Func {}

enum Expr<I, F: Func + ?Sized> {
    Integer(I),
    Function {
        caller: Box<F>,
        args: Vec<Expr<I, dyn Func>>
    }
}