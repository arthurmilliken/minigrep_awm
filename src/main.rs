use std::env;
use std::process;

use minigrep_awm::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--

    if let Err(e) = minigrep_awm::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
