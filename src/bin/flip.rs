use std::fs::OpenOptions;
use std::path::PathBuf;

use structopt::StructOpt;

use flipfile::*;

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
/// Flip the bytes in multiple files
struct Opts {
    /// Verbosity
    #[structopt(short, long)]
    verbose: bool,

    /// Use mmap
    #[cfg(feature = "memmap")]
    #[structopt(short, long)]
    mmap: bool,

    /// Files to process
    #[structopt()]
    paths: Vec<PathBuf>,
}

fn main() {
    let opts = Opts::from_args();

    if opts.verbose {
        println!("{:?}", opts);
    }

    for path in opts.paths {
        if opts.verbose {
            println!("processing {:?}", path);
        }

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
                        if opts.verbose {
                            println!("flipped {} bytes", nbytes);
                        }
                    }
                    Err(e) => eprintln!("error while processing {:?}: {}", path, e),
                }
            }
            Err(e) => eprintln!("cannot open {:?}: {}", path, e),
        }
    }
}
