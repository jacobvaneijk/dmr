// (c) Jacob van Eijk <jacob.vaneijk@gmail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => return Err(why.description().to_string())
    };

    let mut src = String::new();

    match file.read_to_string(&mut src) {
        Ok(_) => return Ok(src),
        Err(why) => return Err(why.description().to_string())
    };
}

pub fn compile_file(filename: &str) -> Result<(), String> {
    let mut src = match read_file(filename) {
        Ok(src) => src,
        Err(why) => return Err(why)
    };

    print!("{}", src);

    Ok(())
}
