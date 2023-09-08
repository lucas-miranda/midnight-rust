use crate::math::Matrix4x4;

pub trait WorldViewProjectionUniforms {
    //fn world(&self) -> Matrix4x4<f32>;

    fn view(&self) -> &Matrix4x4<f32>;
    fn mut_view(&mut self) -> &mut Matrix4x4<f32>;

    //fn projection(&self) -> Matrix4x4<f32>;
}
