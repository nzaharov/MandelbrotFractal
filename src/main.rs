use std::path::PathBuf;
use structopt::StructOpt;

pub mod image;
pub mod rect;
use crate::image::*;
use crate::rect::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "640x320")]
    size: ImageSize,
    #[structopt(short, long, default_value = "-2.0:2.0:-2.0:2.0")]
    rect: Rect,
    #[structopt(short, long, default_value = "1")]
    tasks: u8,
    #[structopt(short, long, default_value = "/asd/zad15.png")]
    output: PathBuf,
    #[structopt(short = "q", long = "quiet")]
    is_quiet: bool,
}

fn main() {
    let args = Cli::from_args();
    println!("{:?}", &args.rect);
}
