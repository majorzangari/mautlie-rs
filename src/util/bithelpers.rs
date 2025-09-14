pub trait BitFunctions {
    type Type;

    fn pop_lsb(&mut self) -> u32;
    fn contains(self, other: Self::Type) -> bool;
}

macro_rules! impl_bit_functions {
    ($type:ty) => {
        impl BitFunctions for $type {
            type Type = $type;

            /// remove and return the index of the least significant bit
            fn pop_lsb(&mut self) -> u32 {
                let out = self.trailing_zeros();
                *self &= *self - 1;
                out
            }

            /// return true if self contains any bits in other
            fn contains(self, other: Self::Type) -> bool {
                (self & other) != 0
            }
        }
    };
}

impl_bit_functions!(i8);
impl_bit_functions!(u8);
impl_bit_functions!(i16);
impl_bit_functions!(u16);
impl_bit_functions!(i32);
impl_bit_functions!(u32);
impl_bit_functions!(i64);
impl_bit_functions!(u64);
impl_bit_functions!(isize);
impl_bit_functions!(usize);
