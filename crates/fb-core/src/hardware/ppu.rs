use tracing::debug;

// Pixel processing unit
#[derive(Debug, Default)]
pub struct PPU {}

impl PPU {
    pub fn new() -> Self {
        debug!("PPU initialized");
        Self::default()
    }

    pub fn tick(&mut self) {
        todo!()
    }
}
