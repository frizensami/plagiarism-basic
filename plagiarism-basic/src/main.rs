mod cli_input;
use plagiarismbasic_lib::{run_plagiarism_checks, AppSettings};

use cli_input::get_cli_input;
fn main() {
    // Read settings for algorithm from cli
    let appsettings: AppSettings = get_cli_input();
    run_plagiarism_checks(&appsettings);
}
