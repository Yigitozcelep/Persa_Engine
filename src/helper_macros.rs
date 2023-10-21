#[macro_export]
macro_rules! impl_op{
    ($struct_name:ident, $op_name:path, $func_name: ident, $field: tt) => {
        impl $op_name for $struct_name {
            type Output = Self;
            #[inline(always)]
            fn $func_name(self, rhs: Self) -> Self::Output {
                Self(self.$field.$func_name(rhs.$field))
            }
        } 
    };
}

#[macro_export]
macro_rules! impl_indv_bit_op{
    ($struct_name:ident, $op_name:path, $func_name: ident, $field: tt) => {
        impl $op_name for $struct_name {
            type Output = Self;
            #[inline(always)]
            fn $func_name(self) -> Self::Output {
                Self(self.$field.$func_name())
            }
        } 
    };
}


#[macro_export]
macro_rules! impl_shift_op {
    ($struct_name:ident, $op_name:path, $func_name:ident, $field:tt) => {
        impl $op_name for $struct_name {
            type Output = Self;
            #[inline(always)]
            fn $func_name(self, rhs: u64) -> Self::Output {
                Self(self.$field.$func_name(rhs))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_all_ops {
    ($struct_name:ident, $field: tt) => {
        impl_op!($struct_name, std::ops::BitAnd, bitand, $field);
        impl_op!($struct_name, std::ops::BitOr, bitor, $field);
        impl_op!($struct_name, std::ops::BitXor, bitxor, $field);
        impl_op!($struct_name, std::ops::Rem, rem, $field);
        impl_op!($struct_name, std::ops::Add, add, $field);
        impl_op!($struct_name, std::ops::Div, div, $field);
        impl_op!($struct_name, std::ops::Mul, mul, $field);
        impl_op!($struct_name, std::ops::Sub, sub, $field);

        impl_indv_bit_op!($struct_name, std::ops::Not, not, $field);
        
        impl_shift_op!($struct_name, std::ops::Shl<u64>, shl, $field);
        impl_shift_op!($struct_name, std::ops::Shr<u64>, shr, $field);
    };
}