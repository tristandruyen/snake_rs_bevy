#![feature(type_ascription)]
#![feature(slice_fill)]

use log::{debug, LevelFilter};
use structopt::StructOpt;

mod game;

#[derive(StructOpt, Debug)]
#[structopt(name = "RGE", version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opt {
    #[structopt(short = "v", parse(from_occurrences), help = "Verbosity")]
    verbosity: u8,
}

fn main() {
    let opt = Opt::from_args();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    let log_level = match opt.verbosity {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 | _ => LevelFilter::Trace,
    };

    // Initialize logging
    initialize_logging(log_level);

    debug!("Debug logging enabled.");

    game::run();
}

fn initialize_logging(our_level_filter: LevelFilter) {
    use chrono::Utc;

    fern::Dispatch::new()
        .level(LevelFilter::Error)
        .level_for("rust_game_engine", our_level_filter)
        .chain(std::io::stderr())
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} | {} | {} | {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.target(),
                record.level(),
                message
            ))
        })
        .apply()
        .unwrap();
}
