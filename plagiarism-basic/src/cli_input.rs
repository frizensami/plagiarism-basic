use clap::{App, Arg};
use plagiarismbasic_lib::{AppSettings, Metric};

pub fn get_cli_input() -> AppSettings {
    let app = App::new("Basic Plagiarism Checker")
        .about("Checks for plagiarism using very basic metrics between different text files")
        .author("Sriram Sami (@frizensami on GitHub)")
        .arg(Arg::with_name("untrusted-directory")
                .short("u")
                .long("untrusted")
                .help("Sets the directory containing untrusted text files. Each file will be treated as a separate submission by a separate person.")
                .takes_value(true)
                .required(true))
        .arg(Arg::with_name("trusted-directory")
                .short("t")
                .long("trusted")
                .help("Sets the directory containing trusted text files. Each file will be treated as a separate possible plagiarism source text.")
                .takes_value(true))
        .arg(Arg::with_name("ignore-directory")
                .short("i")
                .long("ignore")
                .help("Sets the directory containing text files with content to be ignored from plagiarism checks.")
                .takes_value(true))
        .arg(Arg::with_name("metric")
                .short("m")
                .long("metric")
                .help("Sets the metric (function) used for similarity testing. Equal checks that both strings are equal, and lev uses the Levenshtein distance")
                .takes_value(true)
                .required(true)
                .possible_values(&["equal", "lev"]))
        .arg(Arg::with_name("sensitivity")
                .short("n")
                .long("sensitivity")
                .help("Sets the number of words required to form a unit of plagiarism checking")
                .takes_value(true)
                .required(true))
        .arg(Arg::with_name("similarity")
                .short("s")
                .long("similarity")
                .help("Sets the threshold value for plagiarism to be detected by a chosen metric")
                .takes_value(true)
                .required(true))
        .arg(Arg::with_name("output-cli")
                .long("cli")
                .help("If the results should be printed to the command line"))
        .arg(Arg::with_name("output-html")
                .long("html")
                .help("If the results should be printed to a HTML file"))
        .arg(Arg::with_name("open-html")
                .long("openhtml")
                .help("If the HTML file should be opened automatically after writing")) ;

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
    let tdir: Option<String> = matches.value_of("trusted-directory").map(|x| x.to_string());
    let idir: Option<String> = matches.value_of("ignore-directory").map(|x| x.to_string());

    // Get flag options
    let output_cli = matches.is_present("output-cli");
    let output_html = matches.is_present("output-html");
    let open_html_after = matches.is_present("open-html");
    AppSettings {
        n,
        s,
        metric,
        udir: udir.to_string(),
        tdir,
        idir,
        output_cli,
        output_html,
        open_html_after,
    }
}
