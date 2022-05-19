use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "quantisation")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
    #[structopt(short, long)]
    colors: usize,
}

fn main() {
    let opt = Opt::from_args();

    quantization::quantize(&opt.input, &opt.output, opt.colors);
}
