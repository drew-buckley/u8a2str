use std::io::Read;

use anyhow::{anyhow, bail, Error};
use argh::FromArgs;

const NULLOUT_PATTERNS: &[&str] = &[
    " ", "\t", "\n", "\r"
];

/// u8a2str: convert string representation of [u8] to its UTF-8 equivalent.
#[derive(Clone, FromArgs)]
struct Args {
    /// input string; if omitted, STDIN will be used as input
    #[argh(positional)]
    input: Option<String>,
}

fn main() -> Result<(), Error> {
    let args: Args = argh::from_env();
    let mut input = if let Some(input) = args.input {
        input
    } else {
        let stdin = std::io::stdin();
        let mut input = Vec::new();
        stdin.lock().read_to_end(&mut input)?;
        String::from_utf8(input)?
    };

    for pattern in NULLOUT_PATTERNS {
        if input.contains(pattern) {
            input = input.replace(pattern, "");
        }
    }

    let mut data = Vec::new();
    if input.starts_with("[") && input.ends_with("]") {
        let tokens = input[1..input.len() - 1].split(",");
        for token in tokens {
            let b = str::parse::<u8>(token).map_err(|e| anyhow!("Failed to parse array context: {e}"))?;
            data.push(b);
        }
    } else {
        bail!("Expected pattern contained within []");
    }

    println!("{}", String::from_utf8_lossy(&data));

    Ok(())
}
