use std::error::Error;

fn main() {
    let v = Vector {
        vect: [1.0, 2.0, 3.0],
        column_vector: true,
    };
    println!("{:?}", v.transpose());
}

//pub trait Differentiable {
//    fn differentiate(&self) -> impl Function;
//}

//pub trait Function {
//    fn eval(&self) -> Vector;
//}

//pub enum Matrix {
//    ColumnVector(ColumnVector),
//}
//

#[derive(Debug)]
pub struct Vector<T: FieldNumber, const SIZE: usize> {
    vect: [T; SIZE],
    column_vector: bool,
}

impl<T: FieldNumber, const SIZE: usize> Vector<T, SIZE> {
    pub fn transpose(self) -> Vector<T, SIZE> {
        Vector {
            vect: self.vect,
            column_vector: !self.column_vector,
        }
    }
}

//pub struct ColumnVector<T: FieldNumber, const SIZE: usize> {}
//
//pub struct RowVector<T: FieldNumber, const SIZE: usize> {
//    vect: [T; SIZE],
//}
//
//impl<T: FieldNumber, const SIZE: usize> Vector for ColumnVector<T, SIZE> {
//    fn transpose(&self) -> impl Vector {
//        RowVector { vect: self.vect }
//    }
//}
//
//impl<T: FieldNumber, const SIZE: usize> Vector for RowVector<T, SIZE> {
//    fn transpose(&self) -> impl Vector {
//        ColumnVector { vect: self.vect }
//    }
//}

pub trait FieldNumber {
    fn zero() -> impl FieldNumber;
    fn one() -> impl FieldNumber;
    fn mult_inverse(&self) -> impl FieldNumber;
    fn add_inverse(&self) -> impl FieldNumber;
}

impl FieldNumber for f64 {
    fn zero() -> impl FieldNumber {
        0.0
    }

    fn one() -> impl FieldNumber {
        1.0
    }

    fn mult_inverse(&self) -> impl FieldNumber {
        1.0 / self
    }

    fn add_inverse(&self) -> impl FieldNumber {
        -self
    }
}

pub struct F2Number {
    val: bool,
}

impl F2Number {
    fn new(bit: u8) -> F2Number {
        match bit > 1 {
            true => panic!("F2 cannot have numbers greater than 1!"),
            false => F2Number {
                val: if bit == 1 { true } else { false },
            },
        }
    }
}

impl FieldNumber for F2Number {
    fn zero() -> impl FieldNumber {
        F2Number::new(0)
    }

    fn one() -> impl FieldNumber {
        F2Number::new(1)
    }

    fn mult_inverse(&self) -> impl FieldNumber {
        F2Number::new(if self.val {
            1
        } else {
            panic!("0 has no multiplicative inverse!")
        })
    }

    fn add_inverse(&self) -> impl FieldNumber {
        F2Number::new(if self.val { 1 } else { 0 })
    }
}

//pub struct RealNumber {
//    val: f64,
//}

//impl FieldNumber for RealNumber {
//    fn zero() -> impl FieldNumber {
//        RealNumber { val: 0.0 }
//    }
//
//    fn one() -> impl FieldNumber {
//        RealNumber { val: 1.0 }
//    }
//
//    fn mult_inverse(&self) -> impl FieldNumber {
//        RealNumber {
//            val: 1.0 / self.val,
//        }
//    }
//
//    fn add_inverse(&self) -> impl FieldNumber {
//        RealNumber { val: -self.val }
//    }
//}
