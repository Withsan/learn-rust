use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;

use crate::pb;
use crate::pb::abi::{filter, filter::Filter, ImageSpec, resize, Resize, Spec, spec};
use crate::pb::abi::resize::{ResizeType, SampleFilter};
use crate::pb::abi::spec::Data::Resize;

mod abi;

impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

/// 生成字符串
impl From<&ImageSpec> for String {
    fn from(image_spec: &ImageSpec) -> Self {
        let data = image_spec.encode_to_vec();
        encode_config(data, URL_SAFE_NO_PAD)
    }
}

// 通过字符串可以创建
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = decode_config(value, URL_SAFE_NO_PAD);
        Ok(ImageSpec::decode(&data[..])?)
    }
}

// 辅助函数，photon_rs相应的方法需要字符串
impl filter::Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Marine => Some("marine"),
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands => Some("islands"),
            filter::Filter::Unspecified => None
        }
    }
}

//与photon_rs的SamplingFilter转换
impl From<SampleFilter> for SamplingFilter {
    fn from(value: resize::SampleFilter) -> Self {
        match value {
            SampleFilter::Undefined => SamplingFilter::Nearest,
            SampleFilter::Nearest => SamplingFilter::Nearest,
            SampleFilter::Triangle => SamplingFilter::Triangle,
            SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            SampleFilter::Gaussian => SamplingFilter::Gaussian,
            SampleFilter::Lanczos3 => SamplingFilter::Lanczos3,
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(Resize(Resize { width, height, rtype: ResizeType::SeamCarve as i32, filter: SampleFilter::Undefined as i32 }))
        }
    }
    pub fn new_resize(width: u32, height: u32, filter: SampleFilter) -> Self {
        Self { data: Some(Resize(Resize { width, height, rtype: ResizeType::Normal as i32, filter: filter as i32 })) }
    }
    pub fn new_filter(filter: Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(pb::abi::Filter { filter: filter as i32 }))
        }
    }
}
