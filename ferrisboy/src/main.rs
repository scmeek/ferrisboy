use std::env;
use std::error::Error;

use ferrisboy_core::add;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match run(&args) {
        Ok(result) => {
            println!("{result}");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn run(args: &[String]) -> Result<u64, Box<dyn Error>> {
    let val1 = args
        .get(1)
        .ok_or("Please enter a number")?
        .parse::<u64>()
        .map_err(|_| "Please enter a number")?;

    let val2 = args
        .get(2)
        .ok_or("Please enter a number")?
        .parse::<u64>()
        .map_err(|_| "Please enter a number")?;

    let result = add(val1, val2);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_with_args() {
        let args = vec!["ferrisboy".to_string(), "3".to_string(), "9".to_string()];
        let result = run(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12);
    }

    #[test]
    fn test_run_no_args() {
        let args = vec!["ferrisboy".to_string()];
        let result = run(&args);
        assert!(result.is_err());
    }
}
