use std::fs::OpenOptions;
use std::path::PathBuf;

use structopt::StructOpt;

use flipfile::*;

#[derive(Debug, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::ColoredHelp,
    after_help = "If none of -f, -r or -s is explicitly passed, then -f is implicitly set.",
    help_message = "Print help information",
    version_message = "Print version information",
)]
/// Flip the bytes in multiple files
struct Options {
    /// Verbosity
    ///
    /// The verbosity can be controlled explicitly by setting the environment variable `RUST_LOG`.
    ///
    /// If `RUST_LOG` is not set, the log level defaults to "info" is --verbose is passed or "warn" otherwise.
    #[structopt(short, long)]
    verbose: bool,

    /// Uses mmap instead of read/write
    #[cfg(feature = "memmap")]
    #[structopt(short, long)]
    mmap: bool,

    /// Flip the bytes, i.e. negate each bit
    #[structopt(short, long)]
    flip: bool,

    /// Reverse the bytes
    #[structopt(short, long)]
    reverse: bool,

    /// Swab the bytes, i.e. swap the first 4 and the last 4 bits
    #[structopt(short, long)]
    swab: bool,

    /// Files to process
    #[structopt()]
    paths: Vec<PathBuf>,
}

fn operations(opts: &Options) -> Operations {
    if !(opts.flip | opts.reverse | opts.swab) {
        Operations {
            flip: true,
            reverse: false,
            swab: false,
        }
    } else {
        Operations {
            flip: opts.flip,
            reverse: opts.reverse,
            swab: opts.swab,
        }
    }
}

fn main() {
    let opts = Options::from_args();

    let default_min_level = if opts.verbose { "info" } else { "warn" };

    let env = env_logger::Env::default().default_filter_or(default_min_level);
    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .format_target(false)
        .init();

    log::debug!("opts = {:?}", opts);

    for path in &opts.paths {
        log::info!("processing {:?}", path);

        match OpenOptions::new().read(true).write(true).open(&path) {
            Ok(mut file) => {
                #[cfg(feature = "memmap")]
                let result = if opts.mmap {
                    process_file_mmap(&mut file, &operations(&opts))
                } else {
                    process_file(&mut file, &operations(&opts))
                };

                #[cfg(not(feature = "memmap"))]
                let result = process_file(&mut file, &operations(&opts));

                match result {
                    Ok(nbytes) => {
                        log::info!(" ??? flipped {} bytes", nbytes);
                    }
                    Err(e) => log::error!("error while processing {:?}: {}", path, e),
                }
            }
            Err(e) => log::error!("cannot open {:?}: {}", path, e),
        }
    }
}
