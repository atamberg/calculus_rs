pub mod f2_number;
pub mod field_number;
pub mod vector;
use vector::Vector;

fn main() {
    let v = Vector::new([1.0, 2.0, 3.0], true);
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
