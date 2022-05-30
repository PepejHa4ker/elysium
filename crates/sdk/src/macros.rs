#[macro_export]
macro_rules! vtable_export {
    ($(
        $(#[$outer:meta])*
        $ident:ident($($arg:ident: $argty:ty),*) -> $output:ty,
    )*$(,)*) => {$(
        $(#[$outer])*
        #[inline]
        pub fn $ident(&self, $($arg: $argty,)*) -> $output {
            unsafe { (self.vtable.$ident)(self, $($arg,)*) }
        }
    )*};
}
