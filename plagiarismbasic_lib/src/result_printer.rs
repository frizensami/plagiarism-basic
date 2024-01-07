use crate::plagiarism_database::PlagiarismResult;

/// Print all untrusted results
pub fn print_results_ut(results: &mut Vec<PlagiarismResult>) {
    results.sort_by(|a, b| {
        b.matching_fragments
            .len()
            .partial_cmp(&a.matching_fragments.len())
            .unwrap()
    });
    println!("\t===== BEGIN UNTRUSTED COMPARISON REPORT (Sorted by decreasing severity) ===== \n");
    for result in results {
        println!(
            "\n\t REPORT: UNTRUSTED ID {} vs UNTRUSTED ID {}",
            result.owner_id1, result.owner_id2
        );
        print_result(result);
    }
    println!("\n\t===== END UNTRUSTED COMPARISON REPORT ===== \n");
}

pub fn print_results_t(results: &mut Vec<PlagiarismResult>) {
    results.sort_by(|a, b| {
        b.matching_fragments
            .len()
            .partial_cmp(&a.matching_fragments.len())
            .unwrap()
    });
    println!("\t**** BEGIN TRUSTED COMPARISON REPORT (Sorted by decreasing severity) **** \n");
    for result in results {
        println!(
            "\n\t REPORT: TRUSTED ID {} vs UNTRUSTED ID {}",
            result.owner_id1, result.owner_id2
        );
        print_result(result);
    }
    println!("\n\t**** END TRUSTED COMPARISON REPORT **** \n");
}

fn print_result(result: &PlagiarismResult) {
    if result.equal_fragments {
        for matching_fragment in &result.matching_fragments {
            println!("Identical fragment detected: {}", matching_fragment.0)
        }
    } else {
        for matching_fragment in &result.matching_fragments {
            println!(
                "Similar fragments detected: {}\nVS\n{}",
                matching_fragment.0, matching_fragment.1
            )
        }
    }
}
