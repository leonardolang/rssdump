extern crate xml;
extern crate regex;
extern crate ansi_term;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use regex::Regex;

use ansi_term::Colour::{Red, Green};

fn main() {
    let fname = match std::env::args().nth(1) {
        Some(fname) => fname,
        None => {
            println!("usage: rssdump <filename>");
            std::process::exit(2);
        }
    };

    let file = File::open(fname).unwrap();
    let file = BufReader::new(file);

    let reg_html = Regex::new("<.+?>").unwrap();
    let reg_spcs = Regex::new("[ \n\t]+").unwrap();

    let colour_wheel = [ Red, Green ];
    let mut colours = colour_wheel.iter().cycle();
    let mut colour = colours.next().unwrap();

    let parser = EventReader::new(file);
    let mut depth = 0;
    let mut doprint = false;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                match (depth, name.local_name.as_str()) {
                    (2, "title") | (2, "published") => {
                        doprint = true
                    },
                    _ => ()
                };
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                match (depth, name.local_name.as_str()) {
                    (2, "entry") => {
                        println!("---");
                        colour = colours.next().unwrap();
                    },
                    _ => ()
                };

                depth -= 1;
            }
            Ok(XmlEvent::Characters(data)) => {
                let data = reg_html.replace_all(&data, "");
                let data = reg_spcs.replace_all(&data, " ");
                if doprint {
                    println!("{}", colour.paint(data));
                    doprint = false;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
