#![recursion_limit = "1024"]

mod fast;
use fast::Fast;
use spinners::{Spinner, Spinners};

use async_stream::stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use std::io::{Error as IoError, ErrorKind::NotFound};

use colored::Colorize;
use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[structopt(long = "count", short = "n", default_value = "1")]
    /// The number of times to run the speed test continuously
    count: usize,
    #[structopt(long = "debug")]
    /// Enable Debug Information
    debug: bool,
}

#[tokio::main]
async fn main() -> CliResult {
    let args = Cli::from_args();
    let mut terminal = term::stdout().ok_or(IoError::new(NotFound, "stdout Not Found"))?;
    let fast = Fast::new().await?;

    let spinner = Spinner::new(&Spinners::Arc, "Starting...".to_string());
    if args.debug {
        eprintln!("\n{:?}", fast);
    }
    let urls = fast.get_urls().await?;
    if args.debug {
        eprintln!("\nURLS: {:?}", urls);
    }

    // A stream of string to out
    let output = stream! {
        yield "Connecting...".to_string();

        let speeds = Fast::measure(urls, args.count, fast.max_payload_length);
        for await kbps in speeds {
            yield format_speed(kbps)
        }
    };
    pin_mut!(output);

    let mut current_message = String::new();
    while let Some(msg) = output.next().await {
        current_message = spinner.message(msg.clone()).unwrap_or(msg);
    }
    spinner.stop();

    terminal.carriage_return()?;
    terminal.delete_line()?;

    println!("{} {}", "✓".green(), current_message);
    Ok(())
}

/// Returns a formatted string with the correct unit measure
fn format_speed(kbps: f64) -> String {
    const YOTTABIT: f64 = 1e24;
    const ZETTABIT: f64 = 1e21;
    const EXABIT: f64 = 1e18;
    const PETABIT: f64 = 1e15;
    const TERABIT: f64 = 1e12;
    const GIGABIT: f64 = 1e9;
    const MEGABIT: f64 = 1e6;
    const KILOBIT: f64 = 1e3;

    let (scale, val) = match kbps * KILOBIT {
        bits if bits > YOTTABIT => ("Y", bits / YOTTABIT),
        bits if bits > ZETTABIT => ("Z", bits / ZETTABIT),
        bits if bits > EXABIT => ("E", bits / EXABIT),
        bits if bits > PETABIT => ("P", bits / PETABIT),
        bits if bits > TERABIT => ("T", bits / TERABIT),
        bits if bits > GIGABIT => ("G", bits / GIGABIT),
        bits if bits > MEGABIT => ("M", bits / MEGABIT),
        bits if bits > KILOBIT => ("K", bits / KILOBIT),
        bits => ("b", bits),
    };
    format!("{:.2} {}bps", val, scale)
}
