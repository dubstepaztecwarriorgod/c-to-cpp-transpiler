use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arg {
    #[arg(short, long, help="The path to the C file")]
    path: PathBuf
}

impl Arg {
    pub fn is_c(&self) {
        let extension = self.path.extension()
    }
}