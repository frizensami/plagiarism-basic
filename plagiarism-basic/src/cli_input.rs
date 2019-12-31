use crate::{AppSettings, Metric};
use clap::{App, Arg};

pub fn get_cli_input() -> AppSettings {
    let app = App::new("Basic Plagiarism Checker")
        .about("Checks for plagiarism using very basic metrics between different text files")
        .version("v0.1")
        .author("Sriram Sami (@frizensami on GitHub)")
        .arg(Arg::with_name("untrusted-directory")
                .short("u")
                .help("Sets the directory containing untrusted text files. Each file will be treated as a separate submission by a separate person.")
                .takes_value(true)
                .required(true))
        .arg(Arg::with_name("trusted-directory")
                .short("t")
                .help("Sets the directory containing trusted text files. Each file will be treated as a separate possible plagiarism source text.")
                .takes_value(true)
                .required(true))
        .arg(Arg::with_name("metric")
                .short("m")
                .help("Sets the metric (function) used for similarity testing. Equal checks that both strings are equal, and lev uses the Levenshtein distance")
                .takes_value(true)
                .required(true)
                .possible_values(&["equal", "lev"]))
        .arg(Arg::with_name("sensitivity")
                .short("n")
                .help("Sets the number of words required to form a unit of plagiarism checking")
                .takes_value(true)
                .required(true))

        .arg(Arg::with_name("similarity")
                .short("s")
                .help("Sets the threshold value for plagiarism to be detected by a chosen metric")
                .takes_value(true)
                .required(true));

    // Get options for algorithm
    let matches = app.get_matches();
    let n: usize = matches.value_of("sensitivity").unwrap().parse().unwrap();
    let s: usize = matches.value_of("similarity").unwrap().parse().unwrap();
    let metricarg: &str = matches.value_of("metric").unwrap();
    let metric: Metric = match metricarg {
        "equal" => Metric::Equal,
        "lev" => Metric::Lev,
        _ => panic!("Incorrect metric argument given!"),
    };

    // Get info from directories
    let udir: &str = matches.value_of("untrusted-directory").unwrap();
    let tdir: &str = matches.value_of("trusted-directory").unwrap();
    AppSettings {
        n,
        s,
        metric,
        udir: udir.to_string(),
        tdir: tdir.to_string(),
    }
}
