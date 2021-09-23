use clap::{App, Arg, ArgMatches};
use std::env;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = argument_handler();
    let (infile, outfile, silent) = arguments_to_vars(matches);

    let reader = create_reader(&infile).unwrap();
    let writer = create_writer(&outfile).unwrap();

    take_input(&silent, reader, writer)
}

fn arguments_to_vars(matches: ArgMatches) -> (String, String, bool) {
    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };
    (infile.to_string(), outfile.to_string(), silent)
}

fn create_writer(outfile: &str) -> Result<Box<dyn Write>> {
    let writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(File::create(outfile)?)
    } else {
        Box::new(io::stdout())
    };
    Ok(writer)
}

fn create_reader(infile: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(File::open(infile)?)
    } else {
        Box::new(io::stdin())
    };
    Ok(reader)
}

fn take_input(silent: &bool, mut reader: Box<dyn Read>, mut writer: Box<dyn Write>) -> Result<()> {
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\rtotal_bytes: {}", total_bytes);
        }
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    if !silent {
        eprintln!("\rtotal_bytes: {}", total_bytes);
    }

    Ok(())
}

fn argument_handler() -> ArgMatches<'static> {
    let matches = App::new("pipeprog")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .help("Write output to a file"),
        )
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();
    matches
}
