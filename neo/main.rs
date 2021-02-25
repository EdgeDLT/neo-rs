use neo::cli::neo::NeoCLI;
use neo::cli::CLI;

use clap::{App, AppSettings};

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let arguments = App::new("neo")
        .version("v0.1.0")
        .about("neo in rust")
        .author("Jinghui Liao <jinghui@wayne.edu>")
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableVersion,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(vec![
            NeoCLI::new(),
        ])
        .set_term_width(0)
        .get_matches();

        NeoCLI::print(NeoCLI::parse(&arguments));
}
