use std::sync::atomic::AtomicU32;

pub struct Framebuffer {
    pub buffer: Vec<AtomicU32 >,
}
