/// validate vtable indices
#[macro_export]
macro_rules! object_validate {
    ($type:ident; $(
        $ident:ident => $offset:literal,
    )*) => {
        #[allow(dead_code)]
        #[allow(invalid_value)]
        const OBJECT_VALIDATION: () = {
            let object: $type = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

            $(
                if frosting::offset_of!(object.$ident) != $offset {
                    panic!(concat!("invalid object.", stringify!($ident), " offset"));
                }
            )*
        };
    };
}

/// reexport vtable functions
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

/// validate vtable indicies
#[macro_export]
macro_rules! vtable_validate {
    ($(
        $ident:ident => $offset:literal,
    )*) => {
        #[allow(dead_code)]
        #[allow(invalid_value)]
        const VTABLE_VALIDATION: () = {
            let vtable: VTable = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

            $(
                if frosting::offset_of!(vtable.$ident) != $offset * 8 {
                    panic!(concat!("invalid vtable.", stringify!($ident), " offset"));
                }
            )*
        };
    };
}
