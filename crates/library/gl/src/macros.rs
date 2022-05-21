#[macro_export]
macro_rules! get_proc {
    ($base:ident.$ident:ident = $get_proc_address:ident($proc:literal)) => {
        $base.$ident = unsafe { core::mem::transmute($get_proc_address($proc)) };

        if $base.$ident.is_none() {
            panic!("{} not found", $proc);
        }
    };
}

#[macro_export]
macro_rules! def_proc {
    (fn $ident:ident(&self, $($arg:ident: $argty:ty),*)) => {
        #[inline]
        fn $ident(&self, $($arg: $argty),*,) {
            //frosting::println!();

            unsafe {
                (self.$ident.unwrap_unchecked())($($arg),*);
            }
        }
    };
    (fn $ident:ident(&self) -> $output:ty) => {
        #[inline]
        #[must_use]
        fn $ident(&self) -> $output {
            //frosting::println!();

            unsafe {
                (self.$ident.unwrap_unchecked())()
            }
        }
    };
    (fn $ident:ident(&self, $($arg:ident: $argty:ty),*) -> $output:ty) => {
        #[inline]
        #[must_use]
        fn $ident(&self, $($arg: $argty),*,) -> $output {
            //frosting::println!();

            unsafe {
                (self.$ident.unwrap_unchecked())($($arg),*)
            }
        }
    };
    (pub fn $ident:ident(&self, $($arg:ident: $argty:ty),*)) => {
        #[inline]
        pub fn $ident(&self, $($arg: $argty),*,) {
            //frosting::println!();

            unsafe {
                (self.$ident.unwrap_unchecked())($($arg),*);
            }
        }
    };
    (pub fn $ident:ident(&self) -> $output:ty) => {
        #[inline]
        #[must_use]
        pub fn $ident(&self) -> $output {
            //frosting::println!();

            unsafe {
                (self.$ident.unwrap_unchecked())()
            }
        }
    };
    (pub fn $ident:ident(&self, $($arg:ident: $argty:ty),*) -> $output:ty) => {
        #[inline]
        #[must_use]
        pub fn $ident(&self, $($arg: $argty),*,) -> $output {
            //frosting::println!();

            unsafe {
                (self.$ident.unwrap_unchecked())($($arg),*)
            }
        }
    };
}
