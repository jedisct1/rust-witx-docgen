#[macro_use]
extern crate clap;

use clap::Arg;
use std::fs::File;
use std::io::{self, Write};
use witx::{Documentation, WitxError};

fn main() -> Result<(), io::Error> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("output_file")
                .short("-o")
                .long("--output")
                .value_name("output_file")
                .multiple(false)
                .help("Output file, or - for the standard output"),
        )
        .arg(
            Arg::with_name("witx_file")
                .multiple(true)
                .required(true)
                .help("wITX file"),
        )
        .get_matches();

    let mut writer: Box<dyn Write> = match matches.value_of("output_file") {
        None | Some("-") => Box::new(std::io::stdout()),
        Some(file) => Box::new(File::create(file).unwrap()),
    };
    let witx_files: Vec<_> = matches.values_of("witx_file").unwrap().collect();
    let witx_doc = match witx::load(&witx_files) {
        Err(WitxError::Io(_, io)) => return Err(io),
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
        Ok(witx_files) => witx_files,
    };
    let md = witx_doc.to_md();
    writer.write_all(md.as_bytes())?;
    Ok(())
}
