use std::fs;
use std::path::Path;
use clap::clap_app;
use hexagony;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap_app!(hexagony =>
        (version: "0.1.0")
        (@group mode +required =>
            (@arg grid: -g [N] "Prints an empty hex grid of side-length N")
            (@arg FILE: "Path to a source file to run")
        )
        (@arg debug: -d "Activates debug annotations in front of the source code")
        (@arg diag: -D "Prints diagnostic information after every program tick")
    ).get_matches();
    // Check for grid argument
    if let Some(s) = matches.value_of("grid") {
        print!("{}", hexagony::source_template(s.parse()?));
        return Ok(());
    }
    // Choose highest debug level that has a flag set
    let debug_level = match (matches.is_present("debug"), matches.is_present("diag")) {
        (_, true) => 2,
        (true, false) => 1,
        (false, false) => 0,
    };
    if let Some(s) = matches.value_of("FILE") {
        let src = fs::read_to_string(Path::new(s))?;
        hexagony::run(&src, debug_level)?;
    }
    Ok(())
}
