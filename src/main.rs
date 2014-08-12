#![feature(macro_rules)]
#![feature(phase)]
#![crate_id="rustdown#0.1"]

extern crate rustdoc;
extern crate getopts;

use std::os;
use std::path::posix::Path;
use getopts::OptGroup;

use html::ToHtml;

mod blocks;
mod mdfile;
mod html;
mod generic;
mod types;
mod convert;


fn print_usage(program: &str, opts: &[OptGroup]) {
    let summary = getopts::short_usage(program, opts);
    let usage = getopts::usage(summary.as_slice(), opts);
    println!("{}", usage);
}

// Get the options.
// Print help if necessary, otherwise return Some(matches).
fn opts() -> Option<getopts::Matches> {
    let args: Vec<String> = os::args();

    let program = "rustdown";

    let opts = [
        getopts::opt("i",
                     "input",
                     "markdown file for input",
                     "INPUT",
                     getopts::Yes,
                     getopts::Req),
        getopts::opt("o",
                     "output",
                     "set output file name",
                     "OUTPUT",
                     getopts::Yes,
                     getopts::Optional),
        getopts::optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => {
            println!("{}\n", f.to_string());
            print_usage(program.as_slice(), opts);
            return None;
        }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return None;
    }
    return Some(matches);
}

fn main() {
    match opts() {
        Some(matches) => {
            let input = matches.opt_str("i").expect("-i argument is required!");
            let input_file = match Path::new_opt(input) {
                Some(file) => file,
                None => return,
            };
            match mdfile::open_markdown_file(&input_file) {
                Ok(file) => {
                    let blocks = blocks::blockify_file(file);
                    println!("{}", blocks);
                    let markdown = convert::convert(blocks);
                    for md in markdown.iter() {
                        let html = md.to_html();
                        println!("{}", html);
                    }
                }
                Err(e) => {
                    println!("Error opening markdown file:\n{}", e);
                }
            }
        }
        _ => {
            // User asked for help
        }
    }
}
