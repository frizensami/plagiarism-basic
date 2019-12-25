mod plagiarism_database;
mod string_compare;
mod text_utils;

use plagiarism_database::PlagiarismDatabase;
use clap::{Arg, App, SubCommand};

/// Overall strategy:
///     Take all input texts and for each:
///         - Remove newlines
///         - Convert to lowercase
///         - Trim
///         - Replace all non-alphanumeric characters with spaces
///         - use str::SplitWhitespace to get only non-whitespace words
///         - collect into vector
///         - take all l-length word sequences and join them
///         - Hash all these word sequences and provide those as well

pub enum Metric {
    Equal,
    Hamming,
    Lev,
}

const TEXT1: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum";

const TEXT2: &str = "enim ad minim veniam, quis nostrud";

fn main() {
;

    let app = App::new("Basic Plagiarism Checker")
        .about("Checks for plagiarism using very basic metrics between different text files")
        .version("v0.1")
        .author("Sriram Sami (@frizensami on GitHub)")
        .arg(Arg::with_name("metric")
                .short("m")
                .help("Sets the metric (function) used for similarity testing. Equal checks that both strings are equal, hamming uses the Hamming function, and lev uses the Levenshtein distance")
                .takes_value(true)
                .required(true)
                .possible_values(&["equal", "hamming", "lev"]))
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

    let matches = app.get_matches();
    let n : usize = matches.value_of("sensitivity").unwrap().parse().unwrap();
    let s : usize = matches.value_of("similarity").unwrap().parse().unwrap();
    let metricarg : &str = matches.value_of("metric").unwrap();
    let metric : Metric = match metricarg {
        "equal" => Metric::Equal,
        "hamming" => Metric::Hamming,
        "lev" => Metric::Lev,
        _ => panic!("Incorrect metric argument given!")
    };

    let mut db = PlagiarismDatabase::new(n, s, metric);
    db.add_untrusted_text("t1".to_string(), TEXT1);
    db.add_untrusted_text("t2".to_string(), TEXT2);
    db.check_untrusted_plagiarism();
}
