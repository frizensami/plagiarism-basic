mod file_utils;
mod plagiarism_database;
mod result_printer;
mod string_compare;
mod text_utils;

use clap::{App, Arg};
use file_utils::get_file_contents_from_dir;
use plagiarism_database::{PlagiarismDatabase, PlagiarismResult};

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
///
#[derive(Copy, Clone, PartialEq)]
pub enum Metric {
    Equal,
    Lev,
}

fn main() {
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
    let untrusted_contents = get_file_contents_from_dir(udir).unwrap();
    let trusted_contents = get_file_contents_from_dir(tdir).unwrap();

    // Add text to the DB
    let mut db = PlagiarismDatabase::new(n, s, metric);

    for (id, val) in untrusted_contents {
        db.add_untrusted_text(id, &val);
    }
    for (id, val) in trusted_contents {
        db.add_trusted_text(id, &val);
    }

    // Add all plagiarism results to a vector of results
    let ut_result: Vec<PlagiarismResult> = db.check_untrusted_plagiarism();
    let t_result: Vec<PlagiarismResult> = db.check_trusted_plagiarism();
    result_printer::print_results_ut(ut_result);
    result_printer::print_results_t(t_result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_equal() {
        assert!(true);
    }
}
