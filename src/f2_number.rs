use crate::field_number::FieldNumber;

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
