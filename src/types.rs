use bass_sys::QWORD;
//use std::os::raw::{c_char, c_void};

// Envelope node
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct BassMixerNode {
    pos: QWORD,
    value: f32,
}

impl BassMixerNode {
    pub fn new(pos: QWORD, value: f32) -> Self {
        Self {
            pos,
            value,
        }
    }
}