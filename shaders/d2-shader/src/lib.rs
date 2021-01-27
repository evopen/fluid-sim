#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr),
    register_attr(spirv)
)]

#[cfg(not(target_arch = "spirv"))]
#[macro_use]
pub extern crate spirv_std_macros;

use spirv_std::glam::{vec2, vec4, Vec2, Vec2Swizzles, Vec4};
use spirv_std::storage_class::{Input, Output};

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main_fs(mut output: Output<Vec4>) {
    output.store(vec4(0.2, 0.6, 1.0, 1.0))
}

#[allow(unused_attributes)]
#[spirv(vertex)]
pub fn main_vs(
    a_pos: Input<Vec2>,
    #[spirv(position)] mut out_pos: Output<Vec4>,
    #[spirv(point_size)] mut out_point_size: Output<f32>,
) {
    let mut a_pos = a_pos.load();
    a_pos /= vec2(800.0, 600.0) * 1.5;
    a_pos = (a_pos * 2.0) - vec2(1.0, 1.0);
    out_pos.store(vec4(a_pos.x, a_pos.y, 0.0, 1.0));
    out_point_size.store(3.0);
}
