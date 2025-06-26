use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

/// A CLI tool for formatting and converting Valve's KeyValues3 (KV3) files.
#[derive(Parser)]
#[command(
    name = "kv3",
    bin_name = "kv3",
    display_name = "kv3",
    author = "The Cursed Apple <to@thecursedapple.com>",
    about,
    long_about = None,
    version = env!("CARGO_PKG_VERSION"),
)]
struct Args {
    /// Path to the input KV3 file to process.
    #[arg(short, long)]
    input: PathBuf,

    /// Action to perform on the input KV3 file.
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Format the KV3 file with standard indentation and line breaks.
    Format {
        /// Path to write the formatted KV3 output (if omitted, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Converts a KV3 file to another specified format.
    Convert {
        /// Target format for conversion (currently only "json" is supported)
        #[arg(short, long, default_value = "json")]
        format: Option<Format>,

        /// Path to write the converted output (if omitted, prints to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    /// JSON format
    Json,
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.input)
        .expect("Failed to read input file. Please check if the file exists and is readable.");
    let kv3_value = kv3::from_str(&input)
        .expect("Failed to parse input file as KV3. Ensure the file is a valid KV3 format.");

    match args.command {
        Command::Format { output } => {
            let kv3_string = kv3::to_string(&kv3_value);
            match output {
                Some(output) => fs::write(output, kv3_string).expect(
                    "Failed to write to output file. Check if the path is valid and writable.",
                ),
                None => println!("{}", kv3_string),
            }
        }

        Command::Convert { format: _, output } => {
            let json_value = kv3::to_json(&kv3_value);
            let json_string =
                serde_json::to_string_pretty(&json_value).expect("Failed to serialize to JSON.");
            match output {
                Some(output) => fs::write(output, json_string).expect(
                    "Failed to write to output file. Check if the path is valid and writable.",
                ),
                None => println!("{}", json_string),
            }
        }
    }
}
