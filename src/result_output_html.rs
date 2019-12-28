use crate::plagiarism_database::{PlagiarismResult, TextOwnerID};
use handlebars::Handlebars;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::File;
use std::process::Command;

#[derive(Serialize)]
struct ResultsHandlebars<'a> {
    results: &'a Vec<PlagiarismResult>,
    texts: HashMap<TextOwnerID, Vec<String>>,
}

/// Outputs results to html
pub fn output_results(
    results: &mut Vec<PlagiarismResult>,
    texts: HashMap<TextOwnerID, Vec<String>>,
) {
    // We want the results by most significant first
    results.sort_by(|a, b| {
        b.matching_fragments
            .len()
            .partial_cmp(&a.matching_fragments.len())
            .unwrap()
    });

    // Goal: display the clean text side by side for each plagiarism result
    //       while highlighting each matching ngram in the same colors

    // We do NOT want to send the entire text over multiple times: could be
    // really large. We want to send over the --locations-- to highlight
    // in the vector of text words

    let json_results = ResultsHandlebars { results, texts };
    let hbars = Handlebars::new();
    let mut source_template = File::open(&"./templates/report.hbs").unwrap();
    create_dir_all("./www/").unwrap();
    let mut output_file = File::create("www/report.html").unwrap();
    hbars
        .render_template_source_to_write(&mut source_template, &json_results, &mut output_file)
        .unwrap();
    println!("Results: \n{}\n", serde_json::to_string(&results).unwrap());
    println!("./www/report.html generated! Opening automatically if Linux...");

    // Open the report using the OS-preferred method if possible
    if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .args(&["./www/report.html"])
            .output()
            .expect("Failed to execute xdg-open!");
    }
}
