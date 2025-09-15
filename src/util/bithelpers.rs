pub trait BitFunctions {
    type Type;

    fn pop_lsb(&mut self) -> u32;
    fn get_lsb(self) -> u32;
    fn clear_lsb(&mut self);
    fn contains(self, other: Self::Type) -> bool;
    fn count_set_bits(self) -> u32;
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

            /// return the index of the least significant bit
            fn get_lsb(self) -> u32 {
                self.trailing_zeros()
            }

            /// removes the least significant bit from self (or does nothing if self is 0)
            fn clear_lsb(&mut self) {
                *self &= *self - 1;
            }

            /// return true if self contains any bits in other
            fn contains(self, other: Self::Type) -> bool {
                (self & other) != 0
            }

            /// return the number of set bits
            fn count_set_bits(self) -> u32 {
                self.count_ones()
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
