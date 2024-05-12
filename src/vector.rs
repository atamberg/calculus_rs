use crate::field_number::FieldNumber;

#[derive(Debug)]
pub struct Vector<T: FieldNumber, const SIZE: usize> {
    vect: [T; SIZE],
    column_vector: bool,
}

impl<T: FieldNumber, const SIZE: usize> Vector<T, SIZE> {
    pub fn new(vect: [T; SIZE], column_vector: bool) -> Vector<T, SIZE> {
        Vector {
            vect,
            column_vector,
        }
    }

    pub fn transpose(self) -> Vector<T, SIZE> {
        Vector {
            vect: self.vect,
            column_vector: !self.column_vector,
        }
    }
}
