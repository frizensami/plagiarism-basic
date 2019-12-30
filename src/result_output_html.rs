use crate::plagiarism_database::{PlagiarismResult, TextOwnerID};
use gcollections::ops::*;
use handlebars::Handlebars;
use interval::interval_set::*;
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

// Send a set of these over for each text display
#[derive(Serialize)]
struct TextMaybeBold {
    text: String,
    is_bold: bool,
}

#[derive(Serialize)]
struct HBPlagiarismResult {
    owner_id1: TextOwnerID,
    owner_id2: TextOwnerID,
    trusted_owner1: bool,
    equal_fragments: bool,
    text_display1: Vec<TextMaybeBold>,
    text_display2: Vec<TextMaybeBold>,
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

    // Calculate the text output for each result
    let mut plag_results: Vec<HBPlagiarismResult> = Vec::new();

    for result in results {
        let mut text1_intervals = IntervalSet::empty();
        let mut text2_intervals = IntervalSet::empty();
        for (text1_locs, text2_locs) in &result.matching_fragments_locations {
            for text1_loc in text1_locs {
                text1_intervals = text1_intervals.union(&text1_loc.to_interval_set());
            }
            for text2_loc in text2_locs {
                text2_intervals = text2_intervals.union(&text2_loc.to_interval_set());
            }
        }
        let mut t1_boldtext: Vec<TextMaybeBold> = Vec::new();
        let mut cur_words: Vec<String> = Vec::new();
        let mut contains_previously = false;
        let t1_text = texts.get(&result.owner_id1).unwrap();
        for (i, item) in t1_text.iter().enumerate() {
            let word = item.clone();
            if text1_intervals.contains(&i) {
                // In interval, should be bold
                if contains_previously {
                    cur_words.push(word);
                } else {
                    t1_boldtext.push(TextMaybeBold {
                        text: cur_words.join(" ").clone(),
                        is_bold: false,
                    });
                    cur_words = Vec::new();
                    cur_words.push(word);
                }
                contains_previously = true;
            } else {
                // Not in interval now
                if contains_previously {
                    t1_boldtext.push(TextMaybeBold {
                        text: cur_words.join(" ").clone(),
                        is_bold: true,
                    });
                    cur_words = Vec::new();
                    cur_words.push(word);
                } else {
                    cur_words.push(word);
                }
                contains_previously = false;
            }
        }
        if !cur_words.is_empty() {
            if contains_previously {
                t1_boldtext.push(TextMaybeBold {
                    text: cur_words.join(" ").clone(),
                    is_bold: true,
                });
            } else {
                t1_boldtext.push(TextMaybeBold {
                    text: cur_words.join(" ").clone(),
                    is_bold: false,
                });
            }
        }

        let mut t2_boldtext: Vec<TextMaybeBold> = Vec::new();
        let mut cur_words: Vec<String> = Vec::new();
        let mut contains_previously = false;
        let t2_text = texts.get(&result.owner_id2).unwrap();
        for (i, item) in t2_text.iter().enumerate() {
            let word = item.clone();
            if text2_intervals.contains(&i) {
                // In interval, should be bold
                if contains_previously {
                    cur_words.push(word);
                } else {
                    t2_boldtext.push(TextMaybeBold {
                        text: cur_words.join(" ").clone(),
                        is_bold: false,
                    });
                    cur_words = Vec::new();
                    cur_words.push(word);
                }
                contains_previously = true;
            } else {
                // Not in interval now
                if contains_previously {
                    t2_boldtext.push(TextMaybeBold {
                        text: cur_words.join(" ").clone(),
                        is_bold: true,
                    });
                    cur_words = Vec::new();
                    cur_words.push(word);
                } else {
                    cur_words.push(word);
                }
                contains_previously = false;
            }
        }
        if !cur_words.is_empty() {
            if contains_previously {
                t2_boldtext.push(TextMaybeBold {
                    text: cur_words.join(" ").clone(),
                    is_bold: true,
                });
            } else {
                t2_boldtext.push(TextMaybeBold {
                    text: cur_words.join(" ").clone(),
                    is_bold: false,
                });
            }
        }

        plag_results.push(HBPlagiarismResult {
            owner_id1: result.owner_id1.clone(),
            owner_id2: result.owner_id2.clone(),
            trusted_owner1: result.trusted_owner1,
            equal_fragments: result.equal_fragments,
            text_display1: t1_boldtext,
            text_display2: t2_boldtext,
        })

        /*
        let t1_intervals_tuples : Vec<BoldInterval> =  text1_intervals
                .iter()
                .map(|x| (bounded::Bounded::lower(x), bounded::Bounded::upper(x)))
                .collect();
        let t2_intervals_tuples : Vec<BoldInterval> =  text2_intervals
                .iter()
                .map(|x| (bounded::Bounded::lower(x), bounded::Bounded::upper(x)))
                .collect();
        let cur_pos1 : usize = 0;
        let t1_interval_idx : usize = 0;
        let t1_text = texts.get(&result.owner_id1).unwrap();
        let mut t1_boldtext : Vec<TextMaybeBold> = Vec::new();

        while cur_pos1 < t1_text.len() && {

        }
        for t1_it in &t1_intervals_tuples {
            let lower = t1_it.0;
            let upper = t1_it.1;

            // All words from that cursor to lowerbound are not bold
            if cur_pos1 < lower {
                let nonbold_text = Vec::from_iter(t1_text[cur_pos1..lower].iter().cloned());
                t1_boldtext.push(TextMaybeBold {
                    text: nonbold_text.join(" "),
                    is_bold: false
                })
            }
        }
        */
        /*
        plag_results.push(hbplagiarismresult {
            owner_id1: result.owner_id1.clone(),
            owner_id2: result.owner_id2.clone(),
            trusted_owner1: result.trusted_owner1,
            equal_fragments: result.equal_fragments,
            text_display1: text1_intervals
                .iter()
                .map(|x| (bounded::bounded::lower(x), bounded::bounded::upper(x)))
                .collect(),
            text_display2: text2_intervals
                .iter()
                .map(|x| (bounded::bounded::lower(x), bounded::bounded::upper(x)))
                .collect(),
        })
        */
    }

    let hbars = Handlebars::new();
    let mut source_template = File::open(&"./templates/report.hbs").unwrap();
    create_dir_all("./www/").unwrap();
    let mut output_file = File::create("www/report.html").unwrap();

    hbars
        .render_template_source_to_write(&mut source_template, &plag_results, &mut output_file)
        .unwrap();

    println!("./www/report.html generated! Opening automatically if Linux...");

    // Open the report using the OS-preferred method if possible
    if cfg!(target_os = "linux") {
        Command::new("xdg-open")
            .args(&["./www/report.html"])
            .output()
            .expect("Failed to execute xdg-open!");
    }
}
