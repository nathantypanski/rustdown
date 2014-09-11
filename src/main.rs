// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(macro_rules)]
#![feature(phase)]
#![feature(globs)]
#![feature(import_shadowing)]

extern crate rustdoc;
extern crate getopts;
extern crate url;

use std::os;
use std::path::posix::Path;
use std::io::fs::File;
use getopts::OptGroup;

use html::ToHtml;

mod blocks;
mod mdfile;
mod html;
mod text;
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
        getopts::optflag("p",
                         "preview",
                         "preview the output in a web browser"),
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

fn read_markdown_file(file: File) {
    let blocks = blocks::blockify_file(file);
    let markdown = convert::convert(blocks);
    for md in markdown.iter() {
        let html = md.to_html();
        println!("{}", html);
    }
}



fn main() {
    match opts() {
        Some(matches) => {
            let input = matches.opt_str("i").expect("Some input argument is required!");
            let input_file = match Path::new_opt(input) {
                Some(file) => file,
                None => return,
            };
            match mdfile::open_markdown_file(&input_file) {
                Ok(file) => {
                    read_markdown_file(file);
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
