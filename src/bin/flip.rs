use std::fs::OpenOptions;
use std::path::PathBuf;

use structopt::StructOpt;

use flipfile::*;

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
/// Flip the bytes in multiple files
struct Opts {
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

    /// Files to process
    #[structopt()]
    paths: Vec<PathBuf>,
}

fn main() {
    let opts = Opts::from_args();

    let default_min_level = if opts.verbose { "info" } else { "warn" };

    let env = env_logger::Env::default().default_filter_or(default_min_level);
    env_logger::Builder::from_env(env)
        .format_timestamp(None)
        .format_target(false)
        .init();

    log::debug!("opts = {:?}", opts);

    for path in opts.paths {
        log::info!("processing {:?}", path);

        match OpenOptions::new().read(true).write(true).open(&path) {
            Ok(mut file) => {
                #[cfg(feature = "memmap")]
                let result = if opts.mmap {
                    flip_file_mmap(&mut file)
                } else {
                    flip_file(&mut file)
                };

                #[cfg(not(feature = "memmap"))]
                let result = flip_file(&mut file);

                match result {
                    Ok(nbytes) => {
                        log::info!(" ↳ flipped {} bytes", nbytes);
                    }
                    Err(e) => log::error!("error while processing {:?}: {}", path, e),
                }
            }
            Err(e) => log::error!("cannot open {:?}: {}", path, e),
        }
    }
}
