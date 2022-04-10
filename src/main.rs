//use bint::{Matrix, Vector};

/*fn main() {
    /*println!("{}", bint::Float::<8, 52>::DIGITS);
    println!("{:b}", f32::EPSILON.to_bits());*/
    let a = f32::from_bits(i32::MAX as u32 - 11);
    let b = f32::from_bits(i32::MAX as u32 - 1);
    println!("{:032b}", (a - b).to_bits());
    println!("a: {:032b}", a.to_bits());
    println!("b: {:032b}", b.to_bits());
    /*println!("{:b}", f64::NAN.to_bits());
    println!("{:b}", (f32::from_bits(1) as f64).to_bits());*/
    let i = 0b01111111101111111111110010001010u32;
    println!("{:032b}", i);
    println!("{:032b}", i.wrapping_add(885));
    let (f1, f2) = (f32::from_bits(i), f32::from_bits(i.wrapping_add(885)));
    assert!(f1.is_nan() && f2.is_nan());
    println!("{:032b}", (f1 - f2).to_bits());

    println!("{}", (f64::from_bits(f64::MIN_POSITIVE.to_bits() + 1) - f64::MIN_POSITIVE).is_subnormal());

    //assert_eq!(5, int_parser::test_proc!(0x80000038479827896789347569873459867349506708937458967395476937458967389054763907456973589476893475689734598673894576893745896738947596789374568));
    let m1: Matrix<i32, 4, 2> = [Vector::from([0, 2, 4, 6]), Vector::from([1, 3, 5, 7])].into();
    let m2: Matrix<i32, 2, 3> = [Vector::from([1, 4]), Vector::from([2, 5]), [3,6].into()].into();
    use core::ops::Mul;
    let m3 = m1.mul(m2);
    println!("{:?}", m3);

    let m1: Matrix<i32, 3, 3> = [Vector::from([1, 0, 0]), Vector::from([4, 0, 0]), Vector::from([0, 1, 0])].into();
    assert!(m1.is_rref());

    let m2: Matrix<i32, 3, 5> = [Vector::from([0, 0, 0]), Vector::from([1, 0, 0]), Vector::from([0, 1, 0]), Vector::from([0, 0, 1]), Vector::from([2, 3, 4])].into();
    assert!(m2.is_rref());

    let m3: Matrix<i32, 4, 4> = [Vector::from([1, 0, 0, 0]), Vector::from([0, 1, 0, 0]), Vector::from([2, 3, 0, 0]), Vector::from([0, 0, 1, 0])].into();
    assert!(m3.is_rref());

    let m4: Matrix<i32, 4, 3> = [Vector::from([0, 1, 0, 0]), Vector::from([0, 4, 0, 0]), Vector::from([0, 0, 1, 0])].into();
    assert!(!m4.is_rref());
}*/

trait Component<P, S> {
    fn state(&self) -> &Rc<RefCell<S>>;
    fn set_state<F: FnMut(&mut S)>(&mut self, mut f: F) {
        f(&mut self.state().borrow_mut());
        self.update();
    }
    fn update(&self);
}

struct MyComp {
    state: Rc<RefCell<(u8, String)>>,
    html: Vec<String>,
}

impl Component<(), (u8, String)> for MyComp {
    fn update(&self) {
        println!("{:?}", self.html);
    }
    fn state(&self) -> &Rc<RefCell<(u8, String)>> {
        &self.state
    }
}

pub struct SharedValue<T> {
    value: Rc<RefCell<T>>,
}

impl<T> SharedValue<T> {
    pub fn set_val<F: FnMut(&mut T)>(&mut self, mut f: F) {
        f(&mut self.value.borrow_mut());
    }
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
        }
    }
    pub fn create_dep(&self) -> Dep<T> {
        Dep::new(self.value.clone())
    }
}

#[derive(Debug)]
pub struct Dep<T> {
    data: Rc<RefCell<T>>,
}

impl<T> Dep<T> {
    pub fn new(data: Rc<RefCell<T>>) -> Self {
        Self { data }
    }
}

impl<T> Clone for Dep<T> {
    fn clone(&self) -> Self {
        Self::new(self.data.clone())
    }
}

use std::rc::Rc;
use std::cell::RefCell;

use bint::I128;

fn main() {
    let mut value = SharedValue::new(String::from("hello"));

    let a = value.create_dep();
    let b = value.create_dep();
    let c = b.clone();

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);


    value.set_val(|s| s.push_str(" world"));

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);


    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    let c = Rc::clone(&value);

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);


    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("{}", I128::from(-136910483565846334909092127208875491329i128).digits()[15] as i8);
    println!("{}", I128::from(-31901471898837980949691369446728269825i128).digits()[15] as i8);
    //int_parser::n!(0x54394587n4466i128);
}