mod file_utils;
mod plagiarism_database;
mod result_output_html;
mod result_printer;
mod string_compare;
mod text_utils;
mod cli_input;

use file_utils::get_file_contents_from_dir;
use plagiarism_database::{PlagiarismDatabase, PlagiarismResult};
use cli_input::get_cli_input;

/// Indicates which metric is being used for plagiarism comparison
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Metric {
    /// Check for equality between strings
    Equal,
    /// Check that Levenshtein distance between strings is lower than
    /// a given bound
    Lev,
}

pub struct AppSettings {
    n: usize,
    s: usize,
    metric: Metric,
    udir: String,
    tdir: String
}

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
fn main() {
    let AppSettings { n, s, metric, udir, tdir }: AppSettings = get_cli_input();
    let untrusted_contents = get_file_contents_from_dir(&udir).unwrap();
    let trusted_contents = get_file_contents_from_dir(&tdir).unwrap();

    // Add text to the DB
    let mut db = PlagiarismDatabase::new(n, s, metric);

    for (id, val) in untrusted_contents {
        db.add_untrusted_text(&id, &val);
    }
    for (id, val) in trusted_contents {
        db.add_trusted_text(&id, &val);
    }

    // Add all plagiarism results to a vector of results
    let mut ut_result: Vec<PlagiarismResult> = db.check_untrusted_plagiarism();
    let mut t_result: Vec<PlagiarismResult> = db.check_trusted_plagiarism();
    result_printer::print_results_ut(&mut ut_result);
    result_printer::print_results_t(&mut t_result);
    ut_result.append(&mut t_result);
    result_output_html::output_results(&mut ut_result, db.get_all_cleantext());
}
