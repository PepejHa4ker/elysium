use core::ptr;

macro_rules! materials {
    ($($variant:ident => ($name:literal, $base:literal, $vdf:expr)),*) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum MaterialKind {
            $($variant,)*
        }

        impl MaterialKind {
            #[inline]
            const fn name_nul_str(&self) -> &'static str {
                match self {
                    $(MaterialKind::$variant => $name,)*
                }
            }

            #[inline]
            const fn base_nul_str(&self) -> &'static str {
                match self {
                    $(MaterialKind::$variant => $name,)*
                }
            }

            #[inline]
            const fn vdf_nul_str(&self) -> Option<&'static str> {
                match self {
                    $(MaterialKind::$variant => $vdf,)*
                }
            }

            /// Material name passed to `Material::new`.
            #[inline]
            pub const fn name(&self) -> &'static str {
                let string = self.name_nul_str();

                unsafe { string.get_unchecked(0..string.len().saturating_sub(1)) }
            }

            /// Material name pointer passed to `Material::new`.
            #[inline]
            pub const fn name_ptr(&self) -> *const u8 {
                self.name_nul_str().as_ptr()
            }

            /// Base VDF/KeyValues object passed to the first argument of `KeyValues::fromString`.
            #[inline]
            pub const fn base(&self) -> &'static str {
                let string = self.base_nul_str();

                unsafe { string.get_unchecked(0..string.len().saturating_sub(1)) }
            }

            /// Base VDF/KeyValues object pointer passed to the first argument of `KeyValues::fromString`.
            #[inline]
            pub const fn base_ptr(&self) -> *const u8 {
                self.base_nul_str().as_ptr()
            }

            /// VDF/KeyValues passed to the second argument of `KeyValues::fromString`.
            #[inline]
            pub const fn vdf(&self) -> Option<&'static str> {
                let string = self.vdf_nul_str()?;

                Some(unsafe {
                    string.get_unchecked(0..string.len().saturating_sub(1))
                })
            }

            /// VDF/KeyValues pointer passed to the second argument of `KeyValues::fromString`.
            #[inline]
            pub const fn vdf_ptr(&self) -> *const u8 {
                match self.vdf_nul_str() {
                    Some(vdf) => vdf.as_ptr(),
                    None => ptr::null(),
                }
            }
        }
    };
}

materials! {
    Normal => ("normal\0", "VertexLitGenric\0", None),
    Flat => ("flat\0", "UnlitGeneric\0", None),
    Chrome => ("chrome\0", "VertexLitGeneric\0", Some("
        $envmap env_cubemap
    \0")),
    Glow => ("glow\0", "VertexLitGeneric\0", Some("
        $additive 1
        $envmap models/effects/cube_white
        $envmapfresnel 1
        $alpha .8
    \0")),
    Pearlescent => ("pearlescent\0", "VertexLitGeneric\0", Some("
        $ambientonly 1
        $phong 1
        $pearlescent 3
        $basemapalphaphongmask 1
    \0")),
    Metallic => ("metallic\0", "VertexLitGeneric\0", Some("
        $basetexture white
        $envmap env_cubemap
        $envmapcontrast 1
        $flat 1
        $halfambert 1
        $ignorez 0
        $model 1
        $nocull 0
        $nofog 1
        $normalmapalphaenvmapmask 1
        $selfillum 1
        $znearer 0
    \0")),
    Animated => ("animated\0", "VertexLitGeneric\0", Some("
        $basetexture dev/zone_warning
        $envmap editor/cube_vertigo
        $envmapcontrast 1
        $envmaptint [.7 .7 .7]
        proxies {
            texturescroll {
                texturescollvar $basetexturetransform
                texturescrollrate 0.6
                texturescrollangle 90
            }
        }
    \0")),
    Platinum => ("platinum\0", "VertexLitGeneric\0", Some("
        $basetexture models/player/ct_fbi/ct_fbi_glass
        $envmap env_cubemap
        $envmaptint [.4 .6 .7]
    \0")),
    Glass => ("glass\0", "VertexLitGeneric\0", Some("
        $additive 1
        $basetexture detail/dt_metal1
        $color [.05 .05 .05]
        $envmap editor/cube_vertigo
    \0")),
    Crystal => ("crystal\0", "VertexLitGeneric\0", Some("
        $basetexture black
        $bumpmap effects/flat_normal
        $envmap models/effects/crystal_cube_vertigo_hdr
        $envmapfresnel 0
        $phong 1
        $phongboost 2
        $phongexponent 16
        $phongtint [.2 .35 .6]
        $translucent 1
    \0")),
    Silver => ("silver\0", "VertexLitGeneric\0", Some("
        $basetexture white
        $bumpmap effects/flat_normal
        $color2 [.05 .05 .05]
        $envmap editor/cube_vertigo
        $envmapfresnel .6
        $envtintmap [.2 .2 .2]
        $phong 1
        $phongboost 2
        $phongexponent 8
        $phongfresnelranges [.7 .8 1]
        $phongtint [.8 .9 1]
    \0")),
    Gold => ("gold\0", "VertexLitGeneric\0", Some("
        $basetexture white
        $bumpmap effects/flat_normal
        $color2 [.18 .15 .06]
        $envmap editor/cube_vertigo
        $envmapfresnel .6
        $envtintmap [.6 .5 .2]
        $phong 1
        $phongboost 6
        $phongdisablehalflambert 1
        $phongexponent 128
        $phongfresnelranges [.7 .8 1]
        $phongtint [.6 .5 .2]
    \0")),
    Plastic => ("plastic\0", "VertexLitGeneric\0", Some("
        $additive 1
        $basetexture black
        $bumpmap models/inventory_items/trophy_majors/matte_metal_normal
        $envmap editor/cube_vertigo
        $envmapfresnel 1
        $normalmapalphaenvmapmask 1
        $phong 1
        $phongboost 20
        $phongdisablehalflambert 1
        $phongexponent 3000
        $phongfesnelranges [.1 .4 1]
        $phongtint [.8 .9 1]
    \0"))
}
