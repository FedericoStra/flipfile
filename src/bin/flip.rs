use std::fs::OpenOptions;
use std::path::PathBuf;

use structopt::StructOpt;

use flipfile::flip_file;

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
/// Flip the bytes in multiple files
struct Opts {
    /// Verbosity
    #[structopt(short, long)]
    verbose: bool,

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
            Ok(mut file) => match flip_file(&mut file) {
                Ok(nbytes) => {
                    if opts.verbose {
                        println!("flipped {} bytes", nbytes);
                    }
                }
                Err(e) => eprintln!("error while processing {:?}: {}", path, e),
            },
            Err(e) => eprintln!("cannot open {:?}: {}", path, e),
        }
    }
}
