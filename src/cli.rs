use crate::image_size::*;
use crate::rect::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Clone)]
pub struct Cli {
    #[structopt(short, long, default_value = "640x480")]
    pub size: ImageSize,
    #[structopt(short, long, default_value = "-2.0:2.0:-2.0:2.0")]
    pub rect: Rect,
    #[structopt(short, long, default_value = "1")]
    pub threads: usize,
    #[structopt(short, long, default_value = "12")]
    pub gran: usize,
    #[structopt(short, long, default_value = "zad15.png")]
    pub output: PathBuf,
    #[structopt(short = "q", long = "quiet")]
    pub is_quiet: bool,
    #[structopt(short = "i", long = "iter", default_value = "1000")]
    pub max_iter: u32,
}

impl Cli {
    pub fn load() -> Self {
        Cli::from_args()
    }
}
