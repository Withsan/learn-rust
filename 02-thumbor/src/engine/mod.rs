mod photon;
use image::ImageOutputFormat;

use crate::pb::Spec;

pub trait Engine {
    // 按照描述处理图片
    fn apply(&mut self, specs: &[Spec]);
    // 生成目标图片
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&self, op: T);
}
