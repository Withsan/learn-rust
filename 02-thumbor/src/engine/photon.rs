use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use photon_rs::native::open_image_from_bytes;
use photon_rs::PhotonImage;

use crate::engine::Engine;
use crate::pb::*;
use crate::Spec;

lazy_static! {
    static ref WATERMARK: PhotonImage ={
        let data=include_bytes!("");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark,64,64,transform::SamplingFilter::Nearest)
    };
}
//元组结构体
pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Corp(ref data)) => self.transform(data),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                Some(spec::Data::Flipv(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v),
                // 对于目前不认识的 spec，不做任何处理
                _ => {}
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

fn image_to_buf(image: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = image.get_raw_pixels();
    let height = image.get_height();
    let width = image.get_width();
    let image_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dyn_image = DynamicImage::ImageRgba8(image_buffer);
    let mut buffer = Vec::with_capacity(32768);
    dyn_image.write_to(&mut buffer, format).unwrap();
    buffer
}
