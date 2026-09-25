use crate::{parser::parse, resolver::Resolver, runtime::Runtime, tokenizer::tokenize};
use std::fs;
mod parser;
mod resolver;
mod runtime;
mod tokenizer;
mod types;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (cmd, path) = match args.as_slice() {
        [_, cmd, path] => (cmd.as_str(), path),
        _ => {
            eprintln!("usage: maybe <run|tokens|statements> <file.mbl>");
            std::process::exit(2);
        }
    };

    let source = fs::read_to_string(path).expect("Could not find the file.");
    let tokens = tokenize(&source);
    let mut resolver = Resolver::new();
    match cmd {
        "tokens" => println!("{tokens:#?}"),
        "statements" => println!("{:#?}", parse(&tokens)),
        "rstatements" => {
            match resolver.resolve_block(&parse(&tokens)) {
                Ok(s) => {
                    println!("{:#?}", s)
                }

                Err(e) => {
                    print!("ERROR: {e:?}");
                    std::process::exit(1);
                }
            };
        }
        "run" => {
            match resolver.resolve_block(&parse(&tokens)) {
                Ok(s) => {
                    let mut runtime = Runtime::new(resolver);
                    match runtime.run(&s) {
                        Ok(_) => std::process::exit(0),
                        Err(e) => {
                            print!("ERROR: {e}");
                            std::process::exit(1);
                        }
                    }
                }

                Err(e) => {
                    print!("ERROR: {e:?}");
                    std::process::exit(1);
                }
            };
        }
        _ => {
            eprintln!("unknown command: {cmd}");
            std::process::exit(2);
        }
    }
}
