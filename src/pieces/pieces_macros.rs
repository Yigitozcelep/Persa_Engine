#[macro_export]
macro_rules! impl_square_index {
    ($struct_name:ident, $output_type:ty, $field:tt) => {
        impl std::ops::Index<Square> for $struct_name {
            type Output = $output_type;
            fn index(&self, index: Square) -> &Self::Output {
                &self.$field[index.0 as usize]
            }
        }
    };
}
