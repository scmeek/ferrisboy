use clap::{Error, Parser, arg};

/// Shared FerrisBoy arguments
#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub debug: bool,

    pub rom_path: String,
}

impl Args {
    pub fn from_args_vec(args: &[String]) -> Result<Self, Error> {
        Args::try_parse_from(args.iter().map(String::as_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM_NAME: &str = "ferrisboy";

    #[test]
    fn test_argument_parsing() {
        let args = vec![
            PROGRAM_NAME.to_string(),
            "test_rom.gb".to_string(),
            "--debug".to_string(),
        ];
        let parsed_args = Args::from_args_vec(&args).unwrap();
        assert_eq!(parsed_args.rom_path, "test_rom.gb");
        assert!(parsed_args.debug);
    }

    #[test]
    fn test_missing_arguments() {
        let args = vec![PROGRAM_NAME.to_string()];
        let parsed_args = Args::from_args_vec(&args);
        assert!(parsed_args.is_err());
    }
}
