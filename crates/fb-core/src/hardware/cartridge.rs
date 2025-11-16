use std::fs;

use tracing::{debug, warn};

use super::cartridge_definitions as defs;
use crate::error::FerrisBoyError;

// TODO: Move to own file?
#[derive(Debug, Default)]
pub struct CartridgeContext {
    path: String,
}

impl CartridgeContext {
    fn new(path: String) -> Self {
        Self { path }
    }

    pub fn filename(&self) -> String {
        self.path.split('/').last().unwrap().to_string()
    }
}

// TODO: Add doc references
/// https://gbdev.io/pandocs/The_Cartridge_Header.html
#[derive(Debug, Default)]
pub struct Cartridge {
    context: CartridgeContext,
    rom: Vec<u8>,
}

impl Cartridge {
    fn new(context: CartridgeContext, rom: Vec<u8>) -> Self {
        let cart = Self { context, rom };
        debug!("Cartridge initialized");
        cart
    }

    pub fn load_from_file(path: &str) -> Result<Self, FerrisBoyError> {
        let context = CartridgeContext::new(path.to_string());

        let bytes = fs::read(path)?;
        debug!("Loaded ROM from file \"{}\" ({} bytes)", path, bytes.len());

        Ok(Self::new(context, bytes))
    }

    pub fn context(&self) -> &CartridgeContext {
        &self.context
    }

    pub fn rom_size(&self) -> usize {
        self.rom.len()
    }

    pub fn check_integrity(&self) -> bool {
        todo!()
    }

    /// Verify FerrisBoy compatibility
    pub fn check_compatibility(&self) -> bool {
        todo!()
    }

    pub fn check_all(&self) -> bool {
        self.check_integrity() && self.check_compatibility()
    }

    pub fn header(&self) -> Option<&[u8]> {
        let bytes = self.rom.get(defs::HEADER_START..=defs::HEADER_END);
        debug!("Cartridge header: {:?}", bytes);
        bytes
    }

    pub fn header_logo(&self) -> Option<&[u8]> {
        let bytes = self
            .rom
            .get(defs::HEADER_LOGO_START..=defs::HEADER_LOGO_END);
        debug!("Cartridge header logo: {:?}", bytes);
        bytes
    }

    // Original length
    pub fn header_title_16(&self) -> Option<String> {
        let bytes = self
            .rom
            .get(defs::HEADER_TITLE_START..=defs::HEADER_TITLE_LEN_16_END);
        match bytes {
            None => {
                warn!("Cartridge header title (16) not found");
                None
            }
            Some(bytes) => {
                let title = bytes.iter().map(|&b| b as char).collect();
                debug!("Cartridge header title (16): {}", title);
                Some(title)
            }
        }
    }

    pub fn header_title_15(&self) -> Option<String> {
        let bytes = self
            .rom
            .get(defs::HEADER_TITLE_START..=defs::HEADER_TITLE_LEN_15_END);
        match bytes {
            None => {
                warn!("Cartridge header title (15) not found");
                None
            }
            Some(bytes) => {
                let title = bytes.iter().map(|&b| b as char).collect();
                debug!("Cartridge header title (15): {}", title);
                Some(title)
            }
        }
    }

    pub fn header_title_11(&self) -> Option<String> {
        let bytes = self
            .rom
            .get(defs::HEADER_TITLE_START..=defs::HEADER_TITLE_LEN_11_END);
        match bytes {
            None => {
                warn!("Cartridge header title (11) not found");
                None
            }
            Some(bytes) => {
                let title = bytes.iter().map(|&b| b as char).collect();
                debug!("Cartridge header title (11): {}", title);
                Some(title)
            }
        }
    }

    pub fn header_manufacturer_code(&self) -> Option<&[u8]> {
        let bytes = self
            .rom
            .get(defs::HEADER_MANUFACTURER_CODE_START..=defs::HEADER_MANUFACTURER_CODE_END);
        debug!("Cartridge header manufacturer code: {:?}", bytes);
        bytes
    }

    pub fn header_color_game_boy_flag(&self) -> Option<defs::ColorGameBoyFlag> {
        let byte = self.rom.get(defs::HEADER_COLOR_GAME_BOY_FLAG);
        match byte {
            None => {
                warn!("Cartridge header color game boy flag not found");
                None
            }
            Some(byte) => {
                let color_game_boy_flag = defs::ColorGameBoyFlag::from_byte(byte);
                debug!(
                    "Cartridge header color game boy flag: {:?}",
                    color_game_boy_flag
                );
                Some(color_game_boy_flag)
            }
        }
    }

    pub fn header_new_licensee(&self) -> defs::NewLicensee {
        let bytes = self
            .rom
            .get(defs::HEADER_NEW_LICENSEE_CODE_START..=defs::HEADER_NEW_LICENSEE_CODE_END);

        let new_licensee = defs::NewLicensee::from_bytes(bytes);
        match new_licensee {
            Ok(new_licensee) => new_licensee,
            Err(error) => {
                warn!("Unknown NewLicensee: {}", error);
                defs::NewLicensee::default()
            }
        }
    }

    pub fn header_super_game_boy_flag(&self) -> Option<defs::SuperGameBoyFlag> {
        let byte = self.rom.get(defs::HEADER_SUPER_GAME_BOY_FLAG);
        match byte {
            None => {
                warn!("Cartridge header super game boy flag not found");
                None
            }
            Some(byte) => {
                let super_game_boy_flag = defs::SuperGameBoyFlag::from_byte(byte);
                debug!(
                    "Cartridge header super game boy flag: {:?}",
                    super_game_boy_flag
                );
                Some(super_game_boy_flag)
            }
        }
    }

    pub fn header_cartridge_type(&self) -> Option<defs::CartridgeType> {
        let byte = self.rom.get(defs::HEADER_CARTRIDGE_TYPE);
        match byte {
            None => {
                warn!("Cartridge header cartridge type not found");
                None
            }
            Some(byte) => {
                let cartridge_type = defs::CartridgeType::from_byte(byte);
                debug!("Cartridge header cartridge type: {:?}", cartridge_type);
                Some(cartridge_type)
            }
        }
    }

    pub fn header_cartridge_rom_size(&self) -> Option<defs::RomSize> {
        let byte = self.rom.get(defs::HEADER_ROM_SIZE);
        match byte {
            None => {
                warn!("Cartridge header ROM size not found");
                None
            }
            Some(byte) => {
                let rom_size = defs::RomSize::from_byte(byte);
                debug!("Cartridge header ROM size: {:?}", rom_size);
                Some(rom_size)
            }
        }
    }

    pub fn header_cartridge_ram_size(&self) -> Option<defs::RamSize> {
        let byte = self.rom.get(defs::HEADER_RAM_SIZE);
        match byte {
            None => {
                warn!("Cartridge header RAM size not found");
                None
            }
            Some(byte) => {
                let ram_size = defs::RamSize::from_byte(byte);
                debug!("Cartridge header RAM size: {:?}", ram_size);
                Some(ram_size)
            }
        }
    }

    pub fn header_destination(&self) -> Option<defs::Destination> {
        let byte = self.rom.get(defs::HEADER_DESTINATION_CODE);
        match byte {
            None => {
                warn!("Cartridge header destination not found");
                None
            }
            Some(byte) => {
                let destination = defs::Destination::from_byte(byte);
                debug!("Cartridge header destination: {:?}", destination);
                Some(destination)
            }
        }
    }
}
