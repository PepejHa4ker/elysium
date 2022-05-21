//! Pre-cached materials used in chams.

use super::{SharedOption, STATE};

macro_rules! materials {
    ($(($get:ident, $set:ident)),*) => {
        pub(crate) struct Materials {
            $($get: SharedOption<*const ()>,)*
        }

        impl Materials {
            #[inline]
            pub const fn new() -> Self {
                Self {
                    $($get: SharedOption::none(),)*
                }
            }
        }

        $(
            #[inline]
            pub unsafe fn $get() -> &'static mut *const () {
                STATE.materials.$get.as_mut()
            }

            #[inline]
            pub fn $set(material: *const ()) {
                unsafe {
                    STATE.materials.$get.write(material);
                }
            }
        )*
    };
}

materials! {
    (crystal, set_crystal),
    (darude, set_darude),
    (flat, set_flat),
    (glass, set_glass),
    (glow, set_glow),
    (gold, set_gold),
    (metallic, set_metallic),
    (oil, set_oil),
    (pearlescent, set_pearlescent),
    (plastic, set_plastic),
    (platinum, set_platinum),
    (silver, set_silver)
}
