use super::STATE;
use core::ptr::NonNull;

macro_rules! materials {
    ($($name:ident),*) => {
        pub(crate) struct Materials {
            $(
                pub $name: NonNull<usize>,
            )*
        }

        impl Materials {
            pub const fn new() -> Self {
                Self {
                    $(
                        $name: NonNull::dangling(),
                    )*
                }
            }
        }

        $(
            #[inline]
            pub unsafe fn $name() -> &'static mut NonNull<usize> {
                &mut STATE.as_mut().materials.$name
            }
        )*
    };
}

materials! {
    crystal,
    darude,
    flat,
    glass,
    glow,
    gold,
    metallic,
    oil,
    pearlescent,
    plastic,
    platinum,
    silver
}
