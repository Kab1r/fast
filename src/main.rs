#![type_length_limit = "1351978"]
#![recursion_limit = "1024"]

mod fast;
use fast::Fast;
use spinners::{Spinner, Spinners};

use async_stream::stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

#[macro_use]
extern crate lazy_static;

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[structopt(long = "count", short = "n", default_value = "1")]
    /// The number of times to run the speed test continuously
    count: usize,
}

#[tokio::main]
async fn main() -> CliResult {
    let args = Cli::from_args();

    // A stream of string to out
    let output = stream! {
        let fast = Fast::new();
        let urls = fast.get_urls().unwrap();
        yield Some("Connecting".to_string());

        let speeds = Fast::measure(urls, args.count, fast.max_payload_length);

        pin_mut!(speeds);
        while let Some(kbps) = speeds.next().await {
            match kbps {
                None => {yield None; break; },
                Some(kbps) => yield Some(format_speed(kbps))
            }
        }
    };
    let spinner = Spinner::new(Spinners::Arc, "Starting".to_string());
    pin_mut!(output);

    let mut last_message = String::new();
    while let Some(msg) = output.next().await {
        if msg.is_none() {
            break;
        }
        let msg = msg.unwrap();
        last_message = msg.clone();
        spinner.message(msg);
    }
    spinner.stop();

    let mut t = term::stdout().expect("Missing Terminal");
    t.carriage_return().unwrap();
    t.delete_line().unwrap();

    println!("○ {}", last_message);
    Ok(())
}

const YOTTABIT: f64 = 1000000000000000000000.;
const ZETTABIT: f64 = 1000000000000000000.;
const EXABIT: f64 = 1000000000000000.;
const PETABIT: f64 = 1000000000000.;
const TERABIT: f64 = 1000000000.;
const GIGABIT: f64 = 1000000.;
const MEGABIT: f64 = 1000.;

/// Returns a formatted string with the correct unit measure
fn format_speed(kbps: f64) -> String {
    match kbps {
        u if u > YOTTABIT => format!("{:.2} Ybps", u / YOTTABIT),
        u if u > ZETTABIT => format!("{:.2} Zbps", u / ZETTABIT),
        u if u > EXABIT => format!("{:.2} Ebps", u / EXABIT),
        u if u > PETABIT => format!("{:.2} Pbps", u / PETABIT),
        u if u > TERABIT => format!("{:.2} Tbps", u / TERABIT),
        u if u > GIGABIT => format!("{:.2} Gbps", u / GIGABIT),
        u if u > MEGABIT => format!("{:.2} Mbps", u / MEGABIT),
        u => format!("{:.2} Kbps", u),
    }
}
