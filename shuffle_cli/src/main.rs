use std::io::Write;
extern crate core;

use std::fs::OpenOptions;
use crate::cli::Cli;

use clap::Parser;
use deadpool::*;
use anyhow::{Context, Result};

pub mod cli;

fn main() -> Result<()> {

    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support()
        .context("Unable to enable ANSI support on Windows")?;

    // Parse and validate CLI arguments
    let mut cli = Cli::parse();
    cli.validate().unwrap();

    let mut pool = Pool::new();
    configure_pool(&mut pool, &cli);

    if pool.is_empty() {
        pool.extend_from_lowercase();
        pool.extend_from_digits();
    }

    // Create or open the file for writing, if cli.output() is specified
    let mut output_file = if let Some(output_path) = cli.output() {
        Some(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(output_path)
                .context("Unable to open or create output file")?,
        )
    } else {
        None
    };

    // Generate and output the passwords
    for _ in 0..cli.count() {
        match pool.generate(cli.length()) {
            Ok(password) => {
                // Save the generated password in a variable
                let generated_password = password.clone();

                // Print the password
                println!("{}", generated_password);

                // Write the password to the file if output is specified
                if let Some(file) = output_file.as_mut() {
                    writeln!(file, "{}", generated_password)
                        .context("Unable to write password to file")?;
                }
            }
            Err(e) => eprintln!("Error generating password: {}", e),
        }
    }

    Ok(())
}


/// Configures the pool based on CLI options.
fn configure_pool(pool: &mut Pool, cli: &Cli) {
    // Extend pool based on CLI flags
    if cli.uppercase { pool.extend_from_uppercase(); }
    if cli.lowercase { pool.extend_from_lowercase(); }
    if cli.digits { pool.extend_from_digits(); }
    if cli.braces { pool.extend_from_braces(); }
    if cli.punctuation { pool.extend_from_punctuation(); }
    if cli.quotes { pool.extend_from_quotes(); }
    if cli.dashes { pool.extend_from_dashes(); }
    if cli.math { pool.extend_from_math(); }
    if cli.logograms { pool.extend_from_logograms(); }

    // Apply exclusions and inclusions
    if let Some(exclude) = cli.exclude() {
        pool.exclude_chars(&exclude);
    }
    if let Some(include) = cli.include() {
        pool.extend_from_string(&include);
    }
}
