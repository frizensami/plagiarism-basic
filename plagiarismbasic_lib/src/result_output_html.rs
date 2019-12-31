use crate::plagiarism_database::{PlagiarismResult, TextOwnerID};
use crate::text_utils::get_boldtext_segments_from_intervals;
use gcollections::ops::*;
use handlebars::Handlebars;
use interval::interval_set::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::File;
use std::process::Command;

// Used to find the templates directory
const TEMPLATE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/report.hbs");

// Send a set of these over for each text display
#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct TextMaybeBold {
    /// The text to to be displayed
    pub text: String,
    /// Should the text be rendered in a bold font?
    pub is_bold: bool,
}

/// A plagiarism result that can be formatted by Handlebars
#[derive(Serialize)]
struct HBPlagiarismResult {
    owner_id1: TextOwnerID,
    owner_id2: TextOwnerID,
    trusted_owner1: bool,
    equal_fragments: bool,
    text_display1: Vec<TextMaybeBold>,
    text_display2: Vec<TextMaybeBold>,
    text1_plag_percent: usize,
    text2_plag_percent: usize,
}

/// Outputs results to html
pub fn output_results(
    results: &mut Vec<PlagiarismResult>,
    texts: HashMap<TextOwnerID, Vec<String>>,
    open_html_after: bool,
) {
    // We want the results by most significant first (most matches)
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
    // in the vector of text words. However, this requires too much handlebars work.

    // Compute the text segments to display and their formatting for each result.
    // This avoids having to figure this out in Handlebars. We lose the goal of
    // sharing the entire text, but reduce complexity in handlebars
    let mut plag_results: Vec<HBPlagiarismResult> = Vec::new();
    for result in results {
        let mut text1_intervals = IntervalSet::empty();
        let mut text2_intervals = IntervalSet::empty();
        // Calculate the union of all locations to bold to minimize the number
        // of fragments we have to send
        for (text1_locs, text2_locs) in &result.matching_fragments_locations {
            for text1_loc in text1_locs {
                text1_intervals = text1_intervals.union(&text1_loc.to_interval_set());
            }
            for text2_loc in text2_locs {
                text2_intervals = text2_intervals.union(&text2_loc.to_interval_set());
            }
        }

        let numwords1 = text1_intervals.iter().fold(0, |acc, inter| {
            acc + (bounded::Bounded::upper(inter) - bounded::Bounded::lower(inter) + 1)
        });
        let numwords2 = text2_intervals.iter().fold(0, |acc, inter| {
            acc + (bounded::Bounded::upper(inter) - bounded::Bounded::lower(inter) + 1)
        });

        // Get the actual text fragments based on the intervals we calculated
        let t1_text = texts.get(&result.owner_id1).unwrap();
        let t1_boldtext: Vec<TextMaybeBold> =
            get_boldtext_segments_from_intervals(&t1_text, &text1_intervals);

        let t2_text = texts.get(&result.owner_id2).unwrap();
        let t2_boldtext: Vec<TextMaybeBold> =
            get_boldtext_segments_from_intervals(&t2_text, &text2_intervals);

        // Add the result to an overall vector to be sent to Handlebars
        plag_results.push(HBPlagiarismResult {
            owner_id1: result.owner_id1.clone(),
            owner_id2: result.owner_id2.clone(),
            trusted_owner1: result.trusted_owner1,
            equal_fragments: result.equal_fragments,
            text_display1: t1_boldtext,
            text_display2: t2_boldtext,
            text1_plag_percent: ((numwords1 as f32) / (t1_text.len() as f32) * 100.0) as usize,
            text2_plag_percent: ((numwords2 as f32) / (t2_text.len() as f32) * 100.0) as usize,
        })
    }

    let hbars = Handlebars::new();
    let mut source_template = File::open(TEMPLATE_PATH).unwrap();
    create_dir_all("./www/").unwrap();
    let mut output_file = File::create("www/report.html").unwrap();

    hbars
        .render_template_source_to_write(&mut source_template, &plag_results, &mut output_file)
        .unwrap();

    // Open the report using the OS-preferred method if possible
    if open_html_after && cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .args(&["./www/report.html"])
            .output()
            .expect("Failed to execute xdg-open!");
    }
}
