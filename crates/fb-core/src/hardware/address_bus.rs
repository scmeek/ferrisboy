use tracing::debug;

// Reading and writing to and from addresses
#[derive(Debug, Default)]
pub struct AddressBus {}

impl AddressBus {
    pub fn new() -> Self {
        debug!("AddressBus initialized");
        Self::default()
    }

    pub fn read(&self, _address: u16) -> u8 {
        todo!()
    }

    pub fn write(&mut self, _address: u16, _data: u8) {
        todo!()
    }
}
