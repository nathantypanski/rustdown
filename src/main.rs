// Copyright 2014 The Rustdown Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(rustdoc)]

extern crate rustdoc;
extern crate getopts;
extern crate url;

use std::env;
use std::path::Path;
use std::fs::File;
use getopts::Options;

use html::ToHtml;

pub mod lines;
pub mod mdfile;
pub mod preprocessor;
pub mod html;
pub mod text;
pub mod types;
pub mod convert;


fn print_usage(program: &str, opts: Options) {
    let summary = opts.short_usage(program);
    let usage = opts.usage(&summary);
    println!("{}", usage);
}

// Get the options.
// Print help if necessary, otherwise return Some(matches).
fn opts() -> Option<getopts::Matches> {
    let args: Vec<String> = env::args().skip(1).collect();

    let program = "rustdown";

    let mut opts = Options::new();
    opts.opt("i",
             "input",
             "markdown file for input",
             "INPUT",
             getopts::HasArg::Yes,
             getopts::Occur::Req);
    opts.opt("o",
             "output",
             "set output file name",
             "OUTPUT",
             getopts::HasArg::Yes,
             getopts::Occur::Optional);
    opts.optflag("p",
                 "preview",
                 "preview the output in a web browser");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(args) {
        Ok(m) => { m }
        Err(f) => {
            println!("{}\n", f.to_string());
            print_usage(&program, opts);
            return None;
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }
    return Some(matches);
}

fn read_markdown_file(file: File) {
    let blocks = preprocessor::replace_tabs(&lines::split_file_lines(file));
    let markdown = convert::parse_block(&blocks);
    let html = markdown.to_html();
    println!("{}", html);
}



fn main() {
    match opts() {
        Some(matches) => {
            let input = matches.opt_str("i").expect("Some input argument is required!");
            let input_file = Path::new(&input);
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
