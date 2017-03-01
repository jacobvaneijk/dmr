// (c) Jacob van Eijk <jacob.vaneijk@gmail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

extern crate getopts;

use getopts::Options;
use std::env;
use std::process;

mod driver;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optflag("h", "help", "Display this information.");
    opts.optflag("V", "version", "Display version information.");
    opts.optopt("o", "", "Place the output into <file>.", "<file>");

    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(why) => {
            println!("{}", why);
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        let usage = format!("Usage: {} [options] file...", &args[0]);
        print!("{}", opts.usage(&usage));
        process::exit(1);
    }

    if matches.opt_present("V") {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }

    if matches.free.is_empty() {
        println!("No files given.");
        process::exit(1);
    }

    let output = match matches.opt_str("o") {
        Some(output) => output,
        None => "a.out".to_string()
    };

    for filename in matches.free {
        match driver::compile_file(&filename) {
            Ok(_) => {},
            Err(why) => {
                print!("{}", why);
                process::exit(1);
            }
        }
    }
}
