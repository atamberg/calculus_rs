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
