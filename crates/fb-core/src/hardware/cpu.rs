use tracing::debug;

// Central processing unit
#[derive(Debug, Default)]
pub struct CPU {}

impl CPU {
    pub fn new() -> Self {
        debug!("CPU initialized");
        Self::default()
    }

    fn _step() -> bool {
        todo!();
    }
}
