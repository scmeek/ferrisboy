use crate::error::FerrisBoyError;
use num_enum::TryFromPrimitive;
use tracing::warn;

// https://gbdev.io/pandocs/The_Cartridge_Header.html
// FIXME: These should probably just return the enums with no Result/Option wrapper and log misses

macro_rules! ascii_pair {
    ($a:literal, $b:literal) => {
        (($a as u16) << 8 | ($b as u16))
    };
}

pub const HEADER_START: usize = 0x0100;
pub const _HEADER_ENTRY_POINT_START: usize = 0x0100;
pub const _HEADER_ENTRY_POINT_END: usize = 0x0103;
pub const HEADER_LOGO_START: usize = 0x0104;
pub const HEADER_LOGO_END: usize = 0x0133;
pub const HEADER_TITLE_START: usize = 0x0134;
pub const HEADER_TITLE_LEN_11_END: usize = 0x013E;
pub const HEADER_TITLE_LEN_15_END: usize = 0x0142;
pub const HEADER_TITLE_LEN_16_END: usize = 0x0143; // Original length
pub const HEADER_MANUFACTURER_CODE_START: usize = 0x013F;
pub const HEADER_MANUFACTURER_CODE_END: usize = 0x0142;
pub const HEADER_COLOR_GAME_BOY_FLAG: usize = 0x0143;
pub const HEADER_NEW_LICENSEE_CODE_START: usize = 0x0144;
pub const HEADER_NEW_LICENSEE_CODE_END: usize = 0x0145;
pub const HEADER_SUPER_GAME_BOY_FLAG: usize = 0x0146;
pub const HEADER_CARTRIDGE_TYPE: usize = 0x0147;
pub const HEADER_ROM_SIZE: usize = 0x0148;
pub const HEADER_RAM_SIZE: usize = 0x0149;
pub const HEADER_DESTINATION_CODE: usize = 0x014A;
pub const HEADER_OLD_LICENSEE_CODE: usize = 0x014B;
pub const HEADER_MASK_ROM_VERSION_NUMBER: usize = 0x014C;
pub const _HEADER_HEADER_CHECKSUM: usize = 0x014D;
pub const _HEADER_GLOBAL_CHECKSUM_START: usize = 0x014E;
pub const _HEADER_GLOBAL_CHECKSUM_END: usize = 0x014F;
pub const HEADER_END: usize = 0x014F;

/// https://gbdev.io/pandocs/The_Cartridge_Header.html#0143--cgb-flag
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ColorGameBoyFlag {
    SupportsAndBackwardsCompatible = 0x80,
    ColorGameBoyOnly = 0xC0,

    #[default]
    Unknown,
}

impl ColorGameBoyFlag {
    pub fn from_byte(byte: &u8) -> Self {
        let flag = Self::try_from(*byte);

        match flag {
            Ok(flag) => flag,
            Err(error) => {
                warn!("Unknown ColorGameBoyFlag: {}", error);
                Self::default()
            }
        }
    }
}

// FIXME: Add footnotes to licensee codes and make sure everything has doc strings
/// https://gbdev.io/pandocs/The_Cartridge_Header.html#01440145--new-licensee-code
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum NewLicensee {
    None = ascii_pair!('0', '0'),

    /// Nintendo Research & Development 1
    NintendoResearchAndDevelopment1 = ascii_pair!('0', '1'),

    Capcom = ascii_pair!('0', '8'),

    /// EA (Electronic Arts)
    ElectronicArts1 = ascii_pair!('1', '3'),

    /// Hudson Soft
    HudsonSoft1 = ascii_pair!('1', '8'),

    /// B-AI
    BAi = ascii_pair!('1', '9'),

    /// KSS
    Kss = ascii_pair!('2', '0'),

    /// Planning Office WADA
    PlanningOfficeWADA = ascii_pair!('2', '2'),

    // PCM Complete
    PCMComplete = ascii_pair!('2', '4'),

    /// San-X
    SanX = ascii_pair!('2', '5'),

    Kemco = ascii_pair!('2', '8'),

    /// SETA Corporation
    SetaCorporation = ascii_pair!('2', '9'),

    Viacom = ascii_pair!('3', '0'),

    Nintendo = ascii_pair!('3', '1'),

    /// Bandai
    Bandai1 = ascii_pair!('3', '2'),

    /// Ocean Software/Acclaim Entertainment
    OceanSoftwareOrAcclaimEntertainment1 = ascii_pair!('3', '3'),

    /// Konami
    Konami1 = ascii_pair!('3', '4'),

    HectorSoft = ascii_pair!('3', '5'),

    Taito = ascii_pair!('3', '7'),

    /// Hudson Soft
    HudsonSoft2 = ascii_pair!('3', '8'),

    Banpresto = ascii_pair!('3', '9'),

    /// Ubi Soft
    UbiSoft = ascii_pair!('4', '1'),

    Atlus = ascii_pair!('4', '2'),

    /// Malibu Interactive
    MalibuInteractive = ascii_pair!('4', '4'),

    Angel = ascii_pair!('4', '6'),

    /// Bullet-Proof Software
    BulletProofSoftware = ascii_pair!('4', '7'),

    Irem = ascii_pair!('4', '9'),

    Absolute = ascii_pair!('5', '0'),

    /// Acclaim Entertainment
    AcclaimEntertainment = ascii_pair!('5', '1'),

    Activision = ascii_pair!('5', '2'),

    /// Sammy USA Corporation
    SammyUSACorporation = ascii_pair!('5', '3'),

    /// Konami
    Konami2 = ascii_pair!('5', '4'),

    /// Hi Tech Expressions
    HiTechExpressions = ascii_pair!('5', '5'),

    /// LJN
    Ljn = ascii_pair!('5', '6'),

    Matchbox = ascii_pair!('5', '7'),

    Mattel = ascii_pair!('5', '8'),

    /// Milton Bradley Company
    MiltonBradleyCompany = ascii_pair!('5', '9'),

    /// Titus Interactive
    TitusInteractive = ascii_pair!('6', '0'),

    /// Virgin Games Ltd.
    VirginGamesLtd = ascii_pair!('6', '1'),

    /// Lucasfilm Games
    LucasfilmGames = ascii_pair!('6', '4'),

    /// Ocean Software
    OceanSoftware = ascii_pair!('6', '7'),

    /// EA (Electronic Arts)
    ElectronicArts2 = ascii_pair!('6', '9'),

    Infogrames = ascii_pair!('7', '0'),

    /// Interplay Entertainment
    InterplayEntertainment = ascii_pair!('7', '1'),

    Broderbund = ascii_pair!('7', '2'),

    /// Sculptured Software
    SculpturedSoftware = ascii_pair!('7', '3'),

    /// The Sales Curve Limited
    TheSalesCurveLimited = ascii_pair!('7', '5'),

    /// THQ
    Thq = ascii_pair!('7', '8'),

    Accolade = ascii_pair!('7', '9'),

    /// Misawa Entertainment
    MisawaEntertainment = ascii_pair!('8', '0'),

    /// lozc
    Lozc = ascii_pair!('8', '3'),

    /// Tokuma Shoten
    TokumaShoten = ascii_pair!('8', '6'),

    /// Tsukuda Original
    TsukudaOriginal = ascii_pair!('8', '7'),

    /// Chunsoft Co.
    ChunsoftCo = ascii_pair!('9', '1'),

    /// Video System
    VideoSystem = ascii_pair!('9', '2'),

    /// Ocean Software/Acclaim Entertainment
    OceanSoftwareOrAcclaimEntertainment2 = ascii_pair!('9', '3'),

    Varie = ascii_pair!('9', '5'),

    /// Yonezawa/s’pal
    YonezawaOrSpal = ascii_pair!('9', '6'),

    Kaneko = ascii_pair!('9', '7'),

    /// Pack-In-Video
    PackInVideo = ascii_pair!('9', '9'),

    /// Bottom Up
    BottomUp = ascii_pair!('9', 'H'),

    /// Konami (Yu-Gi-Oh!)
    KonamiYuGiOh = ascii_pair!('A', '4'),

    /// MTO
    Mto = ascii_pair!('B', 'L'),

    Kodansha = ascii_pair!('D', 'K'),

    #[default]
    Unknown,
}

impl NewLicensee {
    pub fn from_word(word: &u16) -> Self {
        let licensee = Self::try_from(*word);
        match licensee {
            Ok(licensee) => licensee,
            Err(error) => {
                warn!("Unknown NewLicensee: {}", error);
                Self::default()
            }
        }
    }

    pub fn from_bytes(bytes: Option<&[u8]>) -> Result<Self, FerrisBoyError> {
        let expected_bytes_length = 2;
        let bytes: &[u8] = match bytes {
            None => {
                return Err(FerrisBoyError::InvalidInput {
                    expected: expected_bytes_length,
                    actual: 0,
                });
            }
            Some(bytes) => bytes,
        };

        {
            let bytes_length = bytes.len();
            if bytes_length != expected_bytes_length {
                return Err(FerrisBoyError::InvalidInput {
                    expected: expected_bytes_length,
                    actual: bytes_length,
                });
            }
        }

        // PERF: Unnecessary copy?
        let new_licensee = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(NewLicensee::from_word(&new_licensee))
    }
}

/// https://gbdev.io/pandocs/The_Cartridge_Header.html#0146--sgb-flag
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum SuperGameBoyFlag {
    SupportsSuperGameBoyFunctions = 0x03,

    #[default]
    Unsupported,
}

impl SuperGameBoyFlag {
    pub fn from_byte(byte: &u8) -> Self {
        let flag = Self::try_from(*byte);

        match flag {
            Ok(flag) => flag,
            Err(error) => {
                warn!("Unknown SuperGameBoyFlag: {}", error);
                Self::default()
            }
        }
    }
}

/// https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum CartridgeType {
    RomOnly = 0x00,

    Mbc1 = 0x01,

    Mbc1AndRam = 0x02,

    Mbc1AndRamAndBattery = 0x03,

    Mbc2 = 0x05,

    Mbc2AndBattery = 0x06,

    /// No licensed cartridge makes use of this option. The exact behavior is unknown.
    RomAndRam = 0x08,

    /// No licensed cartridge makes use of this option. The exact behavior is unknown.
    RomAndRamAndBattery = 0x09,

    Mmm01 = 0x0B,

    Mmm01AndRam = 0x0C,

    Mmm01AndRamAndBattery = 0x0D,

    Mbc3AndTimerAndBattery = 0x0F,

    /// MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version (the Japanese version of Pokémon Crystal Version).
    Mbc3AndTimerAndRamAndBattery = 0x10,

    Mbc3 = 0x11,

    /// MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version (the Japanese version of Pokémon Crystal Version).
    Mbc3AndRam = 0x12,

    /// MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version (the Japanese version of Pokémon Crystal Version).
    Mbc3AndRamAndBattery = 0x13,

    Mbc5 = 0x19,

    Mbc5AndRam = 0x1A,

    Mbc5AndRamAndBattery = 0x1B,

    Mbc5AndRumble = 0x1C,

    Mbc5AndRumbleAndRam = 0x1D,

    Mbc5AndRumbleAndRamAndBattery = 0x1E,

    Mbc6 = 0x20,

    Mbc7AndSensorAndRumbleAndRamAndBattery = 0x22,

    PocketCamera = 0xFC,

    BandaiTama5 = 0xFD,

    HuC3 = 0xFE,

    HuC1AndRamAndBattery = 0xFF,

    // Arbitrarily setting to an unused value
    #[default]
    Unknown = 0xFB,
}

impl CartridgeType {
    pub fn from_byte(byte: &u8) -> Self {
        let cartridge_type = Self::try_from(*byte);

        match cartridge_type {
            Ok(cartridge_type) => cartridge_type,
            Err(error) => {
                warn!("Unknown CartridgeType: {}", error);
                Self::default()
            }
        }
    }
}

/// https://gbdev.io/pandocs/The_Cartridge_Header.html#0148--rom-size
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum RomSize {
    _32KiB = 0x00,
    _64KiB = 0x01,
    _128KiB = 0x02,
    _256KiB = 0x03,
    _512KiB = 0x04,
    _1MiB = 0x05,
    _2MiB = 0x06,
    _4MiB = 0x07,
    _8MiB = 0x08,

    /// Only listed in unofficial docs. No cartridges or ROM files using these sizes are known. As
    /// the other ROM sizes are all powers of 2, these are likely inaccurate. The source of these
    /// values is unknown.
    _1_1MiB = 0x52,

    /// Only listed in unofficial docs. No cartridges or ROM files using these sizes are known. As
    /// the other ROM sizes are all powers of 2, these are likely inaccurate. The source of these
    /// values is unknown.
    _1_2MiB = 0x53,

    /// Only listed in unofficial docs. No cartridges or ROM files using these sizes are known. As
    /// the other ROM sizes are all powers of 2, these are likely inaccurate. The source of these
    /// values is unknown.
    _1_5MiB = 0x54,

    #[default]
    Unknown,
}

impl RomSize {
    pub fn from_byte(byte: &u8) -> Self {
        let rom_size = Self::try_from(*byte);

        match rom_size {
            Ok(rom_size) => rom_size,
            Err(error) => {
                warn!("Unknown RomSize: {}", error);
                Self::default()
            }
        }
    }

    pub fn number_of_rom_banks(&self) -> usize {
        match self {
            Self::_32KiB => 2,
            Self::_64KiB => 4,
            Self::_128KiB => 8,
            Self::_256KiB => 16,
            Self::_512KiB => 32,
            Self::_1MiB => 64,
            Self::_2MiB => 128,
            Self::_4MiB => 256,
            Self::_8MiB => 512,

            // Only listed in unofficial docs. No cartridges or ROM files using these sizes are
            // known. As the other ROM sizes are all powers of 2, these are likely inaccurate. The
            // source of these values is unknown.
            Self::_1_1MiB => 72,

            // Only listed in unofficial docs. No cartridges or ROM files using these sizes are
            // known. As the other ROM sizes are all powers of 2, these are likely inaccurate. The
            // source of these values is unknown.
            Self::_1_2MiB => 80,

            // Only listed in unofficial docs. No cartridges or ROM files using these sizes are
            // known. As the other ROM sizes are all powers of 2, these are likely inaccurate. The
            // source of these values is unknown.
            Self::_1_5MiB => 96,
            Self::Unknown => 0,
        }
    }
}

/// https://gbdev.io/pandocs/The_Cartridge_Header.html#0149--ram-size
/// If the CartridgeType does not include "RAM" in its name, this should be set to 0. This
/// includes `MBC`, since its 512 x 4 bits of memory are built directly into the mapper.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum RamSize {
    _0 = 0x00,

    // Listed in various unofficial docs as 2KiB. However, a 2 KiB RAM chip was never used in
    // a cartridge. The source of this value is unknown.
    Unused = 0x01,
    _8KiB = 0x02,
    _32KiB = 0x03,
    _128KiB = 0x04,
    _64KiB = 0x05,

    #[default]
    Unknown,
}

impl RamSize {
    pub fn from_byte(byte: &u8) -> Self {
        let ram_size = Self::try_from(*byte);

        match ram_size {
            Ok(ram_size) => ram_size,
            Err(error) => {
                warn!("Unknown RamSize: {}", error);
                Self::default()
            }
        }
    }

    pub fn number_of_ram_8_kib_banks(&self) -> usize {
        match self {
            Self::_0 => 0,
            Self::Unused => 0,
            Self::_8KiB => 1,
            Self::_32KiB => 4,
            Self::_128KiB => 16,
            Self::_64KiB => 8,
            Self::Unknown => 0,
        }
    }
}

/// https://gbdev.io/pandocs/The_Cartridge_Header.html#014a--destination-code
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum Destination {
    JapanAndPossiblyOverseas = 0x00,
    OverseasOnly = 0x01,

    #[default]
    Unknown,
}

impl Destination {
    pub fn from_byte(byte: &u8) -> Self {
        let destination = Self::try_from(*byte);

        match destination {
            Ok(destination) => destination,
            Err(error) => {
                warn!("Unknown Destination: {}", error);
                Self::default()
            }
        }
    }
}

// FIXME: Add doc strings from document comments/citations
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum OldLicensee {
    None = ascii_pair!('0', '0'),

    /// Nintendo
    Nintendo1 = ascii_pair!('0', '1'),

    /// Capcom
    Capcom1 = ascii_pair!('0', '8'),

    /// HOT-B
    HotB = ascii_pair!('0', '9'),

    // Jaleco
    Jaleco1 = ascii_pair!('0', 'A'),

    /// Coconuts Japan
    CoconutsJapan = ascii_pair!('0', 'B'),

    /// Elite Systems
    EliteSystems1 = ascii_pair!('0', 'C'),

    /// EA (Electronic Arts)
    ElectronicArts1 = ascii_pair!('1', '3'),

    /// Hudson Soft
    HudsonSoft = ascii_pair!('1', '8'),

    /// ITC Entertainment
    ItcEntertainment = ascii_pair!('1', '9'),

    Yanoman = ascii_pair!('1', 'A'),

    /// Japan Clary
    JapanClary = ascii_pair!('1', 'D'),

    /// Virgin Games Ltd.
    VirginGamesLtd1 = ascii_pair!('1', 'F'),

    /// PCM Complete
    PcmComplete = ascii_pair!('2', '4'),

    /// San-X
    SanX = ascii_pair!('2', '5'),

    /// Kemco
    Kemco1 = ascii_pair!('2', '8'),

    /// SETA Corporation
    SetaCorporation = ascii_pair!('2', '9'),

    /// Infogrames
    Infogrames1 = ascii_pair!('3', '0'),

    Nintendo2 = ascii_pair!('3', '1'),

    /// Bandai
    Bandai2 = ascii_pair!('3', '2'),

    /// Indicates that the New licensee code should be used instead
    NewLicenseeCodeIndicator = ascii_pair!('3', '3'),

    /// Konami
    Konami1 = ascii_pair!('3', '4'),

    HectorSoft = ascii_pair!('3', '5'),

    /// Capcom
    Capcom2 = ascii_pair!('3', '8'),

    /// Banpresto
    Banpresto1 = ascii_pair!('3', '9'),

    /// Entertainment Interactive (stub)
    EntertainmentInteractive = ascii_pair!('3', 'C'),

    Gremlin = ascii_pair!('3', 'E'),

    UbiSoft = ascii_pair!('4', '1'),

    /// Atlus
    Atlus1 = ascii_pair!('4', '2'),

    /// Malibu Interactive
    MalibuInteractive1 = ascii_pair!('4', '4'),

    /// Angel
    Angel1 = ascii_pair!('4', '6'),

    /// Spectrum HoloByte
    SpectrumHoloByte = ascii_pair!('4', '7'),

    Irem = ascii_pair!('4', '9'),

    VirginGamesLtd2 = ascii_pair!('4', 'A'),

    /// Malibu Interactive
    MalibuInteractive2 = ascii_pair!('4', 'D'),

    /// U.S. Gold
    UsGold = ascii_pair!('4', 'F'),

    Absolute = ascii_pair!('5', '0'),

    /// Acclaim Entertainment
    AcclaimEntertainment1 = ascii_pair!('5', '1'),

    Activision = ascii_pair!('5', '2'),

    /// Sammy USA Corporation
    SammyUsaCorporation = ascii_pair!('5', '3'),

    GameTek = ascii_pair!('5', '4'),

    /// Park Place
    ParkPlace = ascii_pair!('5', '5'),

    /// LJN
    Ljn1 = ascii_pair!('5', '6'),

    Matchbox = ascii_pair!('5', '7'),

    /// Milton Bradley Company
    MiltonBradleyCompany = ascii_pair!('5', '9'),

    Mindscape = ascii_pair!('5', 'A'),

    Romstar = ascii_pair!('5', 'B'),

    /// Naxat Soft
    NaxatSoft = ascii_pair!('5', 'C'),

    Tradewest = ascii_pair!('5', 'D'),

    /// Titus Interactive
    TitusInteractive = ascii_pair!('6', '0'),

    /// Virgin Games Ltd.
    VirginGamesLtd = ascii_pair!('6', '1'),

    /// Ocean Software
    OceanSoftware = ascii_pair!('6', '7'),

    /// EA (Electronic Arts)
    ElectronicArts2 = ascii_pair!('6', '9'),

    /// Elite Systems
    EliteSystems2 = ascii_pair!('6', 'E'),

    /// Electro Brain
    ElectroBrain = ascii_pair!('6', 'F'),

    /// Infogrames
    Infogrames2 = ascii_pair!('7', '0'),

    /// Interplay Entertainment
    InterplayEntertainment = ascii_pair!('7', '1'),

    /// Broderbund
    Broderbund1 = ascii_pair!('7', '2'),

    /// Sculptured Software
    SculpturedSoftware = ascii_pair!('7', '3'),

    /// The Sales Curve Limited
    TheSalesCurveLimited = ascii_pair!('7', '5'),

    Thq = ascii_pair!('7', '8'),

    Accolade = ascii_pair!('7', '9'),

    /// Triffix Entertainment
    TriffixEntertainment = ascii_pair!('7', 'A'),

    MicroProse = ascii_pair!('7', 'C'),

    /// Kemco
    Kemco2 = ascii_pair!('7', 'F'),

    /// Misawa Entertainment
    MisawaEntertainment = ascii_pair!('8', '0'),

    /// LOZC G.
    LozcG = ascii_pair!('8', '3'),

    /// Tokuma Shoten
    TokumaShoten1 = ascii_pair!('8', '6'),

    /// Bullet-Proof Software
    BulletProofSoftware = ascii_pair!('8', 'B'),

    /// Vic Tokai Corp.
    VicTokaiCorp = ascii_pair!('8', 'C'),

    /// Ape Inc.
    ApeInc = ascii_pair!('8', 'E'),

    /// I’Max
    Imax = ascii_pair!('8', 'F'),

    /// Chunsoft Co.
    ChunsoftCo = ascii_pair!('9', '1'),

    /// Video System
    VideoSystem = ascii_pair!('9', '2'),

    /// Tsubaraya Productions
    TsubarayaProductions = ascii_pair!('9', '3'),

    /// Varie
    Varie1 = ascii_pair!('9', '5'),

    /// Yonezawa/S’Pal
    YonezawaOrSpal = ascii_pair!('9', '6'),

    /// Kemco
    Kemco3 = ascii_pair!('9', '7'),

    Arc = ascii_pair!('9', '9'),

    /// Nihon Bussan
    NihonBussan = ascii_pair!('9', 'A'),

    Tecmo = ascii_pair!('9', 'B'),

    Imagineer = ascii_pair!('9', 'C'),

    /// Banpresto
    Banpresto2 = ascii_pair!('9', 'D'),

    Nova = ascii_pair!('9', 'F'),

    /// Hori Electric
    HoriElectric = ascii_pair!('A', '1'),

    /// Bandai
    Bandai3 = ascii_pair!('A', '2'),

    /// Konami
    Konami2 = ascii_pair!('A', '4'),

    Kawada = ascii_pair!('A', '6'),

    Takara = ascii_pair!('A', '7'),

    /// Technos Japan
    TechnosJapan = ascii_pair!('A', '9'),

    /// Broderbund
    Broderbund2 = ascii_pair!('A', 'A'),

    /// Toei Animation
    ToeiAnimation = ascii_pair!('A', 'C'),

    Toho = ascii_pair!('A', 'D'),

    Namco = ascii_pair!('A', 'F'),

    /// Acclaim Entertainment
    AcclaimEntertainment2 = ascii_pair!('B', '0'),

    /// ASCII Corporation or Nexsoft
    AsciiCorporationOrNexsoft = ascii_pair!('B', '1'),

    /// Bandai
    Bandai4 = ascii_pair!('B', '2'),

    /// Square Enix
    SquareEnix = ascii_pair!('B', '4'),

    /// HAL Laboratory
    HalLaboratory = ascii_pair!('B', '6'),

    SNK = ascii_pair!('B', '7'),

    /// Pony Canyon
    PonyCanyon1 = ascii_pair!('B', '9'),

    /// Culture Brain
    CultureBrain = ascii_pair!('B', 'A'),

    Sunsoft = ascii_pair!('B', 'B'),

    /// Sony Imagesoft
    SonyImagesoft = ascii_pair!('B', 'D'),

    /// Sammy Corporation
    SammyCorporation = ascii_pair!('B', 'F'),

    /// Taito
    Taito1 = ascii_pair!('C', '0'),

    Kemco = ascii_pair!('C', '2'),

    Square = ascii_pair!('C', '3'),

    /// Tokuma Shoten
    TokumaShoten2 = ascii_pair!('C', '4'),

    /// Data East
    DataEast = ascii_pair!('C', '5'),

    /// Tonkin House
    TonkinHouse = ascii_pair!('C', '6'),

    Koei = ascii_pair!('C', '8'),

    /// UFL
    Ufl = ascii_pair!('C', '9'),

    /// Ultra Games
    UltraGames = ascii_pair!('C', 'A'),

    /// VAP, Inc.
    VapInc = ascii_pair!('C', 'B'),

    /// Use Corporation
    UseCorporation = ascii_pair!('C', 'C'),

    Meldac = ascii_pair!('C', 'D'),

    /// Pony Canyon
    PonyCanyon2 = ascii_pair!('C', 'E'),

    /// Angel
    Angel2 = ascii_pair!('C', 'F'),

    /// Taito
    Taito2 = ascii_pair!('D', '0'),

    /// SOFEL (Software Engineering Lab)
    Sofel = ascii_pair!('D', '1'),

    Quest = ascii_pair!('D', '2'),

    /// Sigma Enterprises
    SigmaEnterprises = ascii_pair!('D', '3'),

    /// ASK Kodansha Co.
    AskKodanshaCo = ascii_pair!('D', '4'),

    /// Naxat Soft14
    NaxatSoft14 = ascii_pair!('D', '6'),

    /// Copya System
    CopyaSystem = ascii_pair!('D', '7'),

    /// Banpresto
    Banpresto3 = ascii_pair!('D', '9'),

    Tomy = ascii_pair!('D', 'A'),

    /// LJN
    Ljn2 = ascii_pair!('D', 'B'),

    /// Nippon Computer Systems
    NipponComputerSystems = ascii_pair!('D', 'D'),

    /// Human Ent.
    HumanEnt = ascii_pair!('D', 'E'),

    Altron = ascii_pair!('D', 'F'),

    /// Jaleco
    Jaleco2 = ascii_pair!('E', '0'),

    /// Towa Chiki
    TowaChiki = ascii_pair!('E', '1'),

    Yutaka = ascii_pair!('E', '2'),

    /// Varie
    Varie2 = ascii_pair!('E', '3'),

    Epoch = ascii_pair!('E', '5'),

    Athena = ascii_pair!('E', '7'),

    /// Asmik Ace Entertainment
    AsmikAceEntertainment = ascii_pair!('E', '8'),

    Natsume = ascii_pair!('E', '9'),

    /// King Records
    KingRecords = ascii_pair!('E', 'A'),

    /// Atlus
    Atlus2 = ascii_pair!('E', 'B'),

    /// Epic/Sony Records
    EpicSonyRecords = ascii_pair!('E', 'C'),

    /// IGS
    Igs = ascii_pair!('E', 'E'),

    /// A Wave
    AWave = ascii_pair!('F', '0'),

    /// Extreme Entertainment
    ExtremeEntertainment = ascii_pair!('F', '3'),

    /// LJN
    Ljn3 = ascii_pair!('F', 'F'),

    #[default]
    Unknown,
}

impl OldLicensee {
    pub fn from_word(word: &u16) -> Self {
        let licensee = Self::try_from(*word);
        match licensee {
            Ok(licensee) => licensee,
            Err(error) => {
                warn!("Unknown OldLicensee: {}", error);
                Self::default()
            }
        }
    }

    pub fn from_bytes(bytes: Option<&[u8]>) -> Result<Self, FerrisBoyError> {
        let expected_bytes_length = 2;
        let bytes: &[u8] = match bytes {
            None => {
                return Err(FerrisBoyError::InvalidInput {
                    expected: expected_bytes_length,
                    actual: 0,
                });
            }
            Some(bytes) => bytes,
        };

        {
            let bytes_length = bytes.len();
            if bytes_length != expected_bytes_length {
                return Err(FerrisBoyError::InvalidInput {
                    expected: expected_bytes_length,
                    actual: bytes_length,
                });
            }
        }

        // PERF: Unnecessary copy?
        let old_licensee = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(OldLicensee::from_word(&old_licensee))
    }
}
