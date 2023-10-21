#[macro_export]
macro_rules! impl_square_index {
    ($struct_name:tt, $output_type:ty, $field:tt) => {
        impl std::ops::Index<Square> for $struct_name {
            type Output = $output_type;
            #[inline(always)]
            fn index(&self, index: Square) -> &Self::Output {
                &self.$field[index.0 as usize]
            }
        }
    };
}

#[macro_export]
macro_rules! impl_square_and_u64_index {
    ($struct_name:ident, $output_type:ty, $field:tt) => {
        impl std::ops::Index<(Square, u64)> for $struct_name {
            type Output = $output_type;
            #[inline(always)]
            fn index(&self, index: (Square, u64)) -> &Self::Output {
                &self.$field[index.1 as usize][index.0.0 as usize]
            }
        }
        
        impl std::ops::IndexMut<(Square, u64)> for $struct_name {
            #[inline(always)]
            fn index_mut(&mut self, index: (Square, u64)) -> &mut Self::Output {
                &mut self.$field[index.1 as usize][index.0.0 as usize]
            }
        }
    };
}