use crate::plagiarism_database::PlagiarismResult;
use handlebars::Handlebars;
use serde::Serialize;
use std::fs::create_dir_all;
use std::fs::File;
use std::process::Command;

#[derive(Serialize)]
struct ResultsHandlebars<'a> {
    results: &'a Vec<PlagiarismResult>,
}

/// Outputs results to html
pub fn output_results(results: &mut Vec<PlagiarismResult>) {
    let json_results = ResultsHandlebars { results: results };
    let hbars = Handlebars::new();
    let mut source_template = File::open(&"./templates/report.hbs").unwrap();
    create_dir_all("./www/").unwrap();
    let mut output_file = File::create("www/report.html").unwrap();
    hbars
        .render_template_source_to_write(&mut source_template, &json_results, &mut output_file)
        .unwrap();
    println!("Results: \n{}\n", serde_json::to_string(&results).unwrap());
    println!("./www/report.html generated! Opening automatically if Linux...");
    if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .args(&["./www/report.html"])
            .output()
            .expect("Failed to execute xdg-open!");
    }
}
