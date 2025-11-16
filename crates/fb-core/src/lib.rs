pub mod arguments;
pub mod error;
pub mod hardware;

use crate::arguments::Args;
use crate::error::FerrisBoyError;
use crate::hardware::{CPU, Cartridge};

pub fn run(args: Args) -> Result<(), FerrisBoyError> {
    let trace_level = if args.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::WARN
    };

    tracing_subscriber::fmt().with_max_level(trace_level).init();

    let cart = Cartridge::load_from_file(&args.rom_path)?;
    cart.header_logo();
    cart.header_title_16();
    cart.header_title_15();
    cart.header_title_11();
    cart.header_manufacturer_code();
    cart.header_color_game_boy_flag();
    cart.header_new_licensee();
    cart.header_super_game_boy_flag();
    cart.header_cartridge_type();
    cart.header_cartridge_rom_size();
    cart.header_cartridge_ram_size();
    cart.header_destination();
    // cart.header_old_licensee();

    cart.check_all();

    let _cpu = CPU::new();

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
