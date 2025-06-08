#[macro_export]
macro_rules! impl_debug_with_field {
    ($struct:ident, $field:ident) => {
        impl std::fmt::Debug for $struct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($struct), "({:?})"), self.$field)
            }
        }
    };
}
