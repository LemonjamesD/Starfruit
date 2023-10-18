use super::VectorType;

use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Vector4<T> {
    pub storage: [T; 4],
    pub vector_type: VectorType
}

impl<T: Copy> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T, vector_type: VectorType) -> Self {
        Self {
            storage: [x, y, z, w],
            vector_type
        }
    }
    pub fn transpose(&mut self) {
        self.vector_type = match self.vector_type {
            VectorType::Row => VectorType::Column,
            VectorType::Column => VectorType::Row
        }
    }
}

impl<T: Display> Display for Vector4<T> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        fmt.write_str((match self.vector_type {
            VectorType::Row => format!("[{} {} {} {}]", self.storage[0], self.storage[1], self.storage[2], self.storage[3]),
            VectorType::Column => format!("┌{}┐\n│{}│\n│{}│\n└{}┘", self.storage[0], self.storage[1], self.storage[2], self.storage[3])
        }).as_str())
    }
}

macro_rules! op_impl {
    ($($op:ident $operator:tt),*) => {
        ::paste::paste! {
            $(
                impl<U: Copy, T: Copy + ::std::ops::$op<U, Output = T>> ::std::ops::$op<U> for Vector4<T> {
                    type Output = Vector4<T>;
            
                    fn [<$op:lower>](self, rhs: U) -> Self::Output {
                        Vector4::new(self.storage[0] $operator rhs, self.storage[1] $operator rhs, self.storage[2] $operator rhs, self.storage[3] $operator rhs, self.vector_type)
                    }
                }
            )*
        }
    }
}

op_impl!(Add +, Sub -, Mul *, Div /);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn operators() {
        let vec = Vector4::new(10, 20, 30, 40, VectorType::Row);
        assert_eq!(vec + 10, Vector4::new(20, 30, 40, 50, VectorType::Row));
        assert_eq!(vec - 10, Vector4::new(0, 10, 20, 30, VectorType::Row));
        assert_eq!(vec * 10, Vector4::new(100, 200, 300, 400, VectorType::Row));
        assert_eq!(vec / 10, Vector4::new(1, 2, 3, 4, VectorType::Row));
    }

    #[test]
    fn display() {
        let vec = Vector4::new(10, 20, 30, 40, VectorType::Row);
        println!("{}", vec);
        let vec = Vector4::new(10, 20, 30, 40, VectorType::Column);
        println!("{}", vec);
    }
}