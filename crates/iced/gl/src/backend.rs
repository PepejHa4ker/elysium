use crate::program;
use crate::quad;
use crate::text;
use crate::triangle;
use crate::{Settings, Transformation, Viewport};

use iced_graphics::backend;
use iced_graphics::font;
use iced_graphics::{Layer, Primitive};
use iced_native::alignment;
use iced_native::{Font, Size};

/// A [`elysium_gl`] graphics backend for [`iced`].
///
/// [`elysium_gl`]: https://github.com/grovesNL/elysium_gl
/// [`iced`]: https://github.com/hecrj/iced
#[derive(Debug)]
pub struct Backend {
    quad_pipeline: quad::Pipeline,
    text_pipeline: text::Pipeline,
    triangle_pipeline: triangle::Pipeline,
    default_text_size: u16,
}

impl Backend {
    /// Creates a new [`Backend`].
    pub fn new(gl: &elysium_gl::Context, settings: Settings) -> Self {
        let text_pipeline =
            text::Pipeline::new(gl, settings.default_font, settings.text_multithreading);

        let shader_version = program::Version::new(gl);

        let quad_pipeline = quad::Pipeline::new(gl, &shader_version);
        let triangle_pipeline = triangle::Pipeline::new(gl, &shader_version);

        Self {
            quad_pipeline,
            text_pipeline,
            triangle_pipeline,
            default_text_size: settings.default_text_size,
        }
    }

    /// Draws the provided primitives in the default framebuffer.
    ///
    /// The text provided as overlay will be rendered on top of the primitives.
    /// This is useful for rendering debug information.
    pub fn present<T: AsRef<str>>(
        &mut self,
        gl: &elysium_gl::Context,
        primitives: &[Primitive],
        viewport: &Viewport,
        overlay_text: &[T],
    ) {
        let viewport_size = viewport.physical_size();
        let scale_factor = viewport.scale_factor() as f32;
        let projection = viewport.projection();

        let mut layers = Layer::generate(primitives, viewport);
        layers.push(Layer::overlay(overlay_text, viewport));

        for layer in layers {
            self.flush(gl, scale_factor, projection, &layer, viewport_size.height);
        }
    }

    fn flush(
        &mut self,
        gl: &elysium_gl::Context,
        scale_factor: f32,
        transformation: Transformation,
        layer: &Layer<'_>,
        target_height: u32,
    ) {
        let mut bounds = (layer.bounds * scale_factor).snap();

        if bounds.width < 1 || bounds.height < 1 {
            return;
        }

        bounds.height = bounds.height.min(target_height);

        if !layer.quads.is_empty() {
            self.quad_pipeline.draw(
                gl,
                target_height,
                &layer.quads,
                transformation,
                scale_factor,
                bounds,
            );
        }

        if !layer.meshes.is_empty() {
            let scaled = transformation * Transformation::scale(scale_factor, scale_factor);

            self.triangle_pipeline
                .draw(gl, target_height, scaled, scale_factor, &layer.meshes);
        }

        if !layer.text.is_empty() {
            for text in layer.text.iter() {
                // Target physical coordinates directly to avoid blurry text
                let text = elysium_gl_glyph::Section {
                    // TODO: We `round` here to avoid rerasterizing text when
                    // its position changes slightly. This can make text feel a
                    // bit "jumpy". We may be able to do better once we improve
                    // our text rendering/caching pipeline.
                    screen_position: (
                        (text.bounds.x * scale_factor).round(),
                        (text.bounds.y * scale_factor).round(),
                    ),
                    // TODO: Fix precision issues with some scale factors.
                    //
                    // The `ceil` here can cause some words to render on the
                    // same line when they should not.
                    //
                    // Ideally, `wgpu_glyph` should be able to compute layout
                    // using logical positions, and then apply the proper
                    // scaling when rendering. This would ensure that both
                    // measuring and rendering follow the same layout rules.
                    bounds: (
                        (text.bounds.width * scale_factor).ceil(),
                        (text.bounds.height * scale_factor).ceil(),
                    ),
                    text: vec![elysium_gl_glyph::Text {
                        text: text.content,
                        scale: elysium_gl_glyph::ab_glyph::PxScale {
                            x: text.size * scale_factor,
                            y: text.size * scale_factor,
                        },
                        font_id: self.text_pipeline.find_font(text.font),
                        extra: elysium_gl_glyph::Extra {
                            color: text.color,
                            z: 0.0,
                        },
                    }],
                    layout: elysium_gl_glyph::Layout::default()
                        .h_align(match text.horizontal_alignment {
                            alignment::Horizontal::Left => elysium_gl_glyph::HorizontalAlign::Left,
                            alignment::Horizontal::Center => {
                                elysium_gl_glyph::HorizontalAlign::Center
                            }
                            alignment::Horizontal::Right => {
                                elysium_gl_glyph::HorizontalAlign::Right
                            }
                        })
                        .v_align(match text.vertical_alignment {
                            alignment::Vertical::Top => elysium_gl_glyph::VerticalAlign::Top,
                            alignment::Vertical::Center => elysium_gl_glyph::VerticalAlign::Center,
                            alignment::Vertical::Bottom => elysium_gl_glyph::VerticalAlign::Bottom,
                        }),
                    ..Default::default()
                };

                self.text_pipeline.queue(text);
            }

            self.text_pipeline.draw_queued(
                gl,
                transformation,
                elysium_gl_glyph::Region {
                    x: bounds.x,
                    y: target_height - (bounds.y + bounds.height),
                    width: bounds.width,
                    height: bounds.height,
                },
            );
        }
    }
}

impl iced_graphics::Backend for Backend {
    fn trim_measurements(&mut self) {
        self.text_pipeline.trim_measurement_cache()
    }
}

impl backend::Text for Backend {
    const ICON_FONT: Font = font::ICONS;
    const CHECKMARK_ICON: char = font::CHECKMARK_ICON;
    const ARROW_DOWN_ICON: char = font::ARROW_DOWN_ICON;

    fn default_size(&self) -> u16 {
        self.default_text_size
    }

    fn measure(&self, contents: &str, size: f32, font: Font, bounds: Size) -> (f32, f32) {
        self.text_pipeline.measure(contents, size, font, bounds)
    }

    fn hit_test(
        &self,
        contents: &str,
        size: f32,
        font: Font,
        bounds: Size,
        point: iced_native::Point,
        nearest_only: bool,
    ) -> Option<text::Hit> {
        self.text_pipeline
            .hit_test(contents, size, font, bounds, point, nearest_only)
    }
}

#[cfg(feature = "image")]
impl backend::Image for Backend {
    fn dimensions(&self, _handle: &iced_native::image::Handle) -> (u32, u32) {
        (50, 50)
    }
}

#[cfg(feature = "svg")]
impl backend::Svg for Backend {
    fn viewport_dimensions(&self, _handle: &iced_native::svg::Handle) -> (u32, u32) {
        (50, 50)
    }
}
