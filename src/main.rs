use std::fs::File;
use std::io::prelude::*;

use pulldown_cmark::{html, Options, Parser};


fn main() {
    let machfile_contents = read_machfile();
    let mut parser = parse_machfile(&machfile_contents);

    // write_html(&mut parser);

    println!("DONE");
}

fn read_machfile() -> String {
    let mut file = File::open("machfile").expect("expected machfile to exist");
    let mut machfile_contents = String::new();
    file.read_to_string(&mut machfile_contents)
        .expect("expected file contents");

    machfile_contents
}

fn parse_machfile<'a>(machfile_contents: &'a String) -> Parser<'a> {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&machfile_contents, options);
    parser
}


fn write_html(parser: &mut Parser) {
    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    let mut file = File::create("machfile.html").unwrap();
    file.write_all(html_output.as_bytes()).unwrap();
}
