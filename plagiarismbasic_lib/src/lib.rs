mod file_utils;
mod plagiarism_database;
mod result_output_html;
mod result_printer;
mod string_compare;
mod text_utils;

use file_utils::get_file_contents_from_dir;
use plagiarism_database::{PlagiarismDatabase, PlagiarismResult};

/// Indicates which metric is being used for plagiarism comparison
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Metric {
    /// Check for equality between strings
    Equal,
    /// Check that Levenshtein distance between strings is lower than
    /// a given bound
    Lev,
}

/// Setting fields parsed by CLI frontend
pub struct AppSettings {
    pub n: usize,
    pub s: usize,
    pub metric: Metric,
    pub udir: String,
    pub tdir: Option<String>,
    pub idir: Option<String>,
    pub output_cli: bool,
    pub output_html: bool,
    pub open_html_after: bool,
}

/// Reads settings from CLI input.
/// Reads all the relevant source files based on settings
/// Loads all sources into DB
/// Runs the plagiarism algorithm with settings from CLI
/// Prints results on CLI
/// Renders results as HTML and opens it automatically using xdg-open if possible
pub fn run_plagiarism_checks(appsettings: &AppSettings) {
    // Read all file contents in both specified directories
    // Fail with panic if any file is not UTF8, or any other error
    let untrusted_contents = get_file_contents_from_dir(&appsettings.udir);

    // Try to add ignore-text if specified. This is required early for optimization.
    let mut ignored_texts: Vec<String> = Vec::new();
    if let Some(idir) = &appsettings.idir {
        let ignore_contents = get_file_contents_from_dir(&idir);
        for (_, val) in ignore_contents {
            ignored_texts.push(val);
        }
    }

    // Add text to the DB
    let mut db = PlagiarismDatabase::new(
        appsettings.n,
        appsettings.s,
        appsettings.metric,
        ignored_texts,
    );

    for (id, val) in untrusted_contents {
        db.add_untrusted_text(&id, &val);
    }

    // Try to add trusted text if specified
    if let Some(tdir) = &appsettings.tdir {
        let trusted_contents = get_file_contents_from_dir(&tdir);
        for (id, val) in trusted_contents {
            db.add_trusted_text(&id, &val);
        }
    }

    // Run both inter-source plagiarism and external-source-based plagiarism checks
    let mut ut_result: Vec<PlagiarismResult> = db.check_untrusted_plagiarism();
    let mut t_result: Vec<PlagiarismResult> = db.check_trusted_plagiarism();

    // Print them separately on the CLI
    if appsettings.output_cli {
        result_printer::print_results_ut(&mut ut_result);
        result_printer::print_results_t(&mut t_result);
    }

    if appsettings.output_html {
        // Pass them together to the HTML output module
        ut_result.append(&mut t_result);
        result_output_html::output_results(
            &mut ut_result,
            db.get_all_cleantext(),
            appsettings.open_html_after,
        );
    }
}
