pub trait BitFunctions {
    type Type;

    fn pop_lsb(&mut self) -> u32;
    fn get_lsb(self) -> u32;
    fn clear_lsb(&mut self);
    fn contains(self, other: Self::Type) -> bool;
    fn count_set_bits(self) -> u32;
    fn bit_for_each<F>(self, func: F)
    where
        F: FnMut(u32);
    fn bit_for_all<F>(self, func: F) -> bool
    where
        F: FnMut(u32) -> bool;
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

            /// runs func for each set bit, passing the index of the bit
            fn bit_for_each<F>(self, mut func: F)
            where
                F: FnMut(u32),
            {
                let mut bb = self;
                while bb != 0 {
                    let lsb_index = bb.pop_lsb();
                    func(lsb_index);
                }
            }

            /// returns true if func returns true for all set bits, false otherwise
            fn bit_for_all<F>(self, mut func: F) -> bool
            where
                F: FnMut(u32) -> bool,
            {
                let mut bb = self;
                while bb != 0 {
                    let lsb_index = bb.pop_lsb();
                    if !func(lsb_index) {
                        return false;
                    }
                }
                true
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
