use clap::App;
use clap::Arg;
use std::error::Error;
use std::path::Path;

mod allele;
mod bcf;
mod bed;
mod configs;
mod partition;

// #[derive(Debug, StructOpt)]
// #[structopt(name = "example", about = "An example of StructOpt usage.")]
// struct Cli{
//
// }
// fn main() {
//     println!("Hello, world!");
// }
//
//

fn main() -> Result<(), Box<dyn Error>> {
    cli()
}

fn check_file_exist_or_exit<P: AsRef<Path>>(file_name: P) {
    let file_name: &Path = file_name.as_ref();
    if !file_name.is_file() {
        clap::Error::with_description(
            &format!("{} doesn't exist", file_name.display()),
            clap::ErrorKind::InvalidValue,
        )
        .exit()
    }
}

fn cli() -> Result<(), Box<dyn Error>> {
    let matches = App::new("jointfish")
        .about("Distributed Joint Genotype Caller")
        .arg(
            Arg::with_name("inputs")
                .value_name("FILE")
                .help("input VCF files")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("bed")
                .short("b")
                .value_name("BED")
                .help("BED file restrict ranges")
                .takes_value(true),
        )
        .get_matches();

    let ranges = matches
        .value_of("bed")
        .map(|path| {
            check_file_exist_or_exit(path);
            bed::range_from_bed_file(path)
        });

    let inputs: Vec<_> = matches
        .values_of("inputs")
        .unwrap()
        .map(|s| Path::new(s))
        .collect();

    // check bcf files
    for input in inputs.iter() {
        if !input.is_file() {
            check_file_exist_or_exit(input);
        }
    }

    let mut bcf = bcf::BcfData::new(inputs.iter().map(|p| p.to_path_buf()).collect::<Vec<_>>());
    bcf.find_contigs_in_all_bcf(&ranges);
    println!("{:?}", bcf.contigs);
    Ok(())
}
