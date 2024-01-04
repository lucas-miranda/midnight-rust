use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Weak,
};

use crate::{
    components::{
        GraphicDisplayer,
        DiagComponent,
    },
    ecs::entity::Entities,
    rendering::{
        fonts::{
            mtsdf::MTSDFShader,
            Font,
        },
        graphics::Text,
        AddressMode,
        FilterMode,
        GraphicAdapter,
        PrimitiveState,
        PrimitiveTopology,
        ShaderConfig,
        Texture,
        TextureConfig,
        TextureSamplerConfig,
        Vertex2DTexture,
    },
};

pub fn create(
    entities: &mut Entities,
    graphic_adapter: &mut GraphicAdapter,
    shader_weak: Weak<RefCell<MTSDFShader>>,
) {
    let mut diag = entities.create();
    diag.register_component(DiagComponent::default());

    diag.register_component({
        let mut graphic_displayer = GraphicDisplayer::<Vertex2DTexture>::default();

        let shader_entry = shader_weak.upgrade().unwrap();
        let mut shader = shader_entry.borrow_mut();

        {
            let distance_range = 2.0f32; //8.0f32;
            //let font_base_size = 32.0;
            //let font_scale = 1.0;
            //let font_size = 32.0;

            let uniforms = shader.uniforms_mut();
            uniforms.color = 0xFF00FFFF.into();
            //uniforms.screen_px_range = ((font_scale * font_size) / font_base_size) * px_distance_range;
            //let unit_range = Vector2::new(px_distance_range, px_distance_range) / 256.0f32;
            uniforms.screen_px_range = distance_range;
        }

        graphic_displayer.shader_config = Some(
            ShaderConfig::new::<MTSDFShader>(
                &shader,
                PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    ..PrimitiveState::default()
                },
            )
        );

        graphic_displayer.texture_config = Some(
            TextureConfig {
                sampler: TextureSamplerConfig {
                    address_mode_u: AddressMode::ClampToEdge,
                    address_mode_v: AddressMode::ClampToEdge,
                    address_mode_w: AddressMode::ClampToEdge,
                    mag_filter: FilterMode::Linear,
                    min_filter: FilterMode::Linear,
                    mipmap_filter: FilterMode::Linear,
                    ..Default::default()
                },
                ..Default::default()
            }
        );

        let font_texture = Texture::load(
            graphic_adapter,
            "fonts/baby.png"
        ).unwrap();

        let data_filepath = PathBuf::from(r"fonts/baby-data.json");
        let font = Font::load_mtsdf(font_texture, data_filepath).with_size(16.0);
        let text = Text::new(font);

        graphic_displayer.graphic = Some(Box::new(text));

        graphic_displayer
    });

    diag.build()
}
