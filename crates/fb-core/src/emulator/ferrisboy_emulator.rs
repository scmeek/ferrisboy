use crate::{emulator::Config, error::Error};

pub struct Emulator {}

impl Emulator {
    #[expect(clippy::missing_errors_doc, reason = "Docs TBD")]
    pub fn new(_config: &Config) -> Result<Self, Error> {
        Ok(Self {})
    }

    // fn run(&self) {}

    // fn shutdown(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emulator_new_success() {
        let config = Config {};
        let result = Emulator::new(&config);
        assert!(result.is_ok());
        let _emulator = result.expect("failed to create emulator");
    }
}
