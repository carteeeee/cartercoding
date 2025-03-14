use three_d::core::*;
use three_d::renderer::*;

#[derive(Clone)]
pub struct MarioMaterial {
    pub texture: Texture2DRef,
    pub render_states: RenderStates,
}

impl Material for MarioMaterial {
    fn id(&self) -> EffectMaterialId {
        EffectMaterialId(0x0001)
    }

    fn fragment_shader_source(&self, lights: &[&dyn Light]) -> String {
        let mut shader = lights_shader_source(lights);
        shader.push_str(ToneMapping::fragment_shader_source());
        shader.push_str(ColorMapping::fragment_shader_source());
        shader.push_str(include_str!("shaders/mario_material.frag"));
        shader
    }

    fn use_uniforms(&self, program: &Program, viewer: &dyn Viewer, lights: &[&dyn Light]) {
        program.use_uniform_if_required("lightingModel", 2u32); // blinn lighting model
        viewer.tone_mapping().use_uniforms(program);
        viewer.color_mapping().use_uniforms(program);
        program.use_uniform_if_required("cameraPosition", viewer.position());
        for (i, light) in lights.iter().enumerate() {
            light.use_uniforms(program, i as u32);
        }
        program.use_uniform("textureTransformation", &self.texture.transformation);
        program.use_texture("tex", &self.texture);
    }

    fn render_states(&self) -> RenderStates {
        self.render_states
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
