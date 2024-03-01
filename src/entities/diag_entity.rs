use std::path::PathBuf;

use crate::{
    base::ApplicationState,
    components::{
        DiagComponent,
        GraphicDisplayer,
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
        PrimitiveState,
        PrimitiveTopology,
        ShaderConfig,
        Texture,
        TextureConfig,
        TextureSamplerConfig,
        Vertex2DTexture,
    }, resources::Asset,
};

pub fn create(
    entities: &mut Entities,
    app_state: &mut ApplicationState,
) {
    let mut diag = entities.create();
    diag.register_component(DiagComponent::default());

    diag.register_component({
        let mut graphic_displayer = GraphicDisplayer::<Vertex2DTexture>::default();

        {
            let shader: &mut MTSDFShader = &mut app_state.asset_resources
                                               .get_mut("default")
                                               .unwrap();

            let distance_range = 2.0f32; //8.0f32;
            //let font_base_size = 32.0;
            //let font_scale = 1.0;
            //let font_size = 32.0;

            // TODO  use vertex color
            let uniforms = shader.uniforms_mut();
            uniforms.color = 0xFF00FFFF.into();
            //uniforms.screen_px_range = ((font_scale * font_size) / font_base_size) * px_distance_range;
            //let unit_range = Vector2::new(px_distance_range, px_distance_range) / 256.0f32;
            uniforms.screen_px_range = distance_range;
        }

        let shader: &MTSDFShader = &app_state.asset_resources
                                           .get("default")
                                           .unwrap();

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

        let font_texture: &Asset<Texture> = &app_state.asset_resources.get_asset("baby").unwrap();

        let data_filepath = PathBuf::from(r"fonts/baby-data.json");
        let font = Font::load_mtsdf(
                font_texture,
                data_filepath
            ).with_size(16.0);
        let text = Text::new(font);

        graphic_displayer.graphic = Some(Box::new(text));

        graphic_displayer
    });

    diag.build()
}
