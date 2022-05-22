use super::Pad;
use frosting::ffi::vtable;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum OverrideKind {
    Normal = 0,
    BuildShadows = 1,
    DepthWrite = 2,
    CustomMaterial = 3,
    SSAODepthWrite = 4,
}

#[repr(C)]
struct VTable {
    _pad0: vtable::Pad<33>,
    forced_material_override: unsafe extern "C" fn(
        this: *const Render,
        material: *const (),
        override_kind: OverrideKind,
        index: i32,
    ),
}

#[repr(C)]
pub struct Render {
    vtable: &'static VTable,
    _pad0: Pad<592>,
    pub material_override: *const (),
    _pad1: Pad<24>,
    pub override_kind: OverrideKind,
}

impl Render {
    #[inline]
    pub fn forced_material_override(
        &self,
        material: *const (),
        override_kind: OverrideKind,
        index: i32,
    ) {
        unsafe {
            (self.vtable.forced_material_override)(self, material, override_kind, index);
        }
    }

    // CStudioRenderContext::IsForcedMaterialOverride
    /*[inline]
    pub fn is_forced_material_override(&self) -> bool {
        if !self.material_override.is_null() {
            return matches!(
                self.override_kind,
                OverrideKind::DepthWrite | OverrideKind::SSAODepthWrite
            );
        }

        self.material_override.name().starts_with("dev/glow")
    }*/
}
