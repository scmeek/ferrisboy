use tracing::debug;

#[derive(Debug, Default)]
pub struct Timer {}

impl Timer {
    pub fn new() -> Self {
        debug!("Timer initialized");
        Self::default()
    }

    pub fn tick(&mut self) {
        todo!()
    }
}
