// features
pub const BLEND: u32 = 0x0BE2;
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const CLAMP_TO_EDGE: u32 = 0x812F;
pub const DYNAMIC_DRAW: u32 = 0x88E8;
pub const FRAMEBUFFER_SRGB: u32 = 0x8DB9;
pub const LINEAR: u32 = 0x2601;
pub const ONE: u32 = 1;
pub const ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const MULTISAMPLE: u32 = 0x809D;
pub const SCISSOR_TEST: u32 = 0x0C11;
pub const SRC_ALPHA: u32 = 0x0302;
pub const TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const TEXTURE_SWIZZLE_RGBA: u32 = 0x8E46;
pub const TEXTURE_WRAP_S: u32 = 0x2802;
pub const TEXTURE_WRAP_T: u32 = 0x2803;
pub const UNPACK_ALIGNMENT: u32 = 0x0CF5;

// colors
pub const RED: u32 = 0x1903;
pub const R8: u32 = 0x8229;

// limits
pub const MAX_TEXTURE_SIZE: u32 = 0x0D33;

// types
pub const ARRAY_BUFFER: u32 = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const FLOAT: u32 = 0x1406;
pub const TEXTURE0: u32 = 0x84C0;
pub const TEXTURE_2D: u32 = 0x0DE1;
pub const UNSIGNED_BYTE: u32 = 0x1401;
pub const UNSIGNED_INT: u32 = 0x1405;

// shapes
pub const TRIANGLES: u32 = 0x0004;
pub const TRIANGLE_STRIP: u32 = 0x0005;

// shaders
pub const FRAGMENT_SHADER: u32 = 0x8B30;
pub const VERTEX_SHADER: u32 = 0x8B31;

// values
pub const ZERO: u32 = 0;

// compilation
pub(crate) const COMPILE_STATUS: u32 = 0x8B81;
pub(crate) const INFO_LOG_LENGTH: u32 = 0x8B84;
pub(crate) const LINK_STATUS: u32 = 0x8B82;
