mod compatibility;
mod core;

use crate::program;
use crate::Transformation;
use iced_graphics::layer;
use iced_native::Rectangle;

#[derive(Debug)]
pub enum Pipeline {
    Core(core::Pipeline),
    Compatibility(compatibility::Pipeline),
}

impl Pipeline {
    pub fn new(gl: &elysium_gl::Context, shader_version: &program::Version) -> Pipeline {
        Pipeline::Core(core::Pipeline::new(gl, shader_version))
    }

    pub fn draw(
        &mut self,
        gl: &elysium_gl::Context,
        target_height: u32,
        instances: &[layer::Quad],
        transformation: Transformation,
        scale: f32,
        bounds: Rectangle<u32>,
    ) {
        match self {
            Pipeline::Core(pipeline) => {
                pipeline.draw(gl, target_height, instances, transformation, scale, bounds);
            }
            Pipeline::Compatibility(pipeline) => {
                pipeline.draw(gl, target_height, instances, transformation, scale, bounds);
            }
        }
    }
}
