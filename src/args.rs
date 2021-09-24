use clap::{App, Arg};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
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

        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let silent = if matches.is_present("silent") {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        };
        Self {
            infile,
            outfile,
            silent,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Args;

    #[test]
    fn test_with_no_cmd_args() {
        let args = Args::parse();
        let Args {
            infile,
            outfile,
            silent,
        } = args;

        assert_eq!(infile, "");
        assert_eq!(outfile, "");
        assert_eq!(silent, false);
    }
}
