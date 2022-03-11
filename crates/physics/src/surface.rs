use sdk::Pad;

#[derive(Debug)]
#[repr(C)]
pub struct Surface {
    pub physics: SurfacePhysics,
    pub audio: SurfaceAudio,
    pub sounds: SurfaceSounds,
    pub properties: SurfaceProperties,
    pub pad: Pad<48>,
}
