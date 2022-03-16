mod sealed {
    pub trait Sealed: Sized {}
}

pub trait Signature: sealed::Sealed {
    type Args;
    type Output;

    unsafe fn call(self, args: Self::Args) -> Self::Output;
}

macro_rules! impl_signature {
    ($($arg:ident),*) => {
        impl<$($arg),*, Output> sealed::Sealed for unsafe extern "C" fn($($arg),*) -> Output {}

        impl<$($arg),*, Output> Signature for unsafe extern "C" fn($($arg),*) -> Output {
            type Args = ($($arg),*,);
            type Output = Output;

            #[allow(non_snake_case)]
            #[inline]
            unsafe fn call(self, args: Self::Args) -> Self::Output {
                let ($($arg),*,) = args;

                (self)($($arg),*)
            }
        }
    };
}

impl_signature!(A);
impl_signature!(A, B);
impl_signature!(A, B, C);
impl_signature!(A, B, C, D);
impl_signature!(A, B, C, D, E);
impl_signature!(A, B, C, D, E, F);
impl_signature!(A, B, C, D, E, F, G);
impl_signature!(A, B, C, D, E, F, G, H);
impl_signature!(A, B, C, D, E, F, G, H, I);
