use wgpu::{CompareFunction, DepthBiasState, DepthStencilState, StencilState};

use crate::image::Texture;

pub fn depth_stencil_state() -> DepthStencilState {
    DepthStencilState {
        format:              Texture::DEPTH_FORMAT,
        depth_write_enabled: true,
        depth_compare:       CompareFunction::Less,
        stencil:             StencilState::default(),
        bias:                DepthBiasState::default(),
    }
}
