use std::{env, fs};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Lex,
    Parse,
    Codegen,
}

struct TokenMatcher {
    name: String,
    patterns: Vec<Regex>,
    token_id: Option<TokenIdentifier>,
}

#[derive(Clone, Debug)]
enum TokenIdentifier {
    Identifier(String),
    Constant(String),
    Keyword(String),
    ParenthesisOpen,
    ParenthesisClose,
    BraceOpen,
    BraceClose,
    Semicolon,
    Comment,
}

fn parse_args(arguments: Vec<String>) -> Result<Command, String> {
    for arg in arguments {
        match arg.as_str() {
            "--lex" => return Ok(Command::Lex),
            "--parse" => return Ok(Command::Parse),
            "--codegen" => return Ok(Command::Codegen),
            _ => continue,
        }
    }

    Err(String::from("no valid option found"))
}

fn lex(src_code_contents: &str, grammer: Vec<TokenMatcher>) -> Option<Vec<TokenIdentifier>> {
    let mut copied_src = src_code_contents.to_string().clone();
    let mut tokens: Vec<TokenIdentifier> = Vec::new();

    loop {
        // remove whitespace at start of string and copy it so we own it
        copied_src = copied_src.clone().trim_start().to_string();
        if copied_src.len() == 0 {
            // if we've read all the source then return the tokens we've collected
            return Some(tokens);
        }

        // keep track of finding a match
        let mut found_match = false;

        // iterate over our grammer patterns, matching and removing the results we find
        for matcher in &grammer {
            // some grammer representations have multiple patterns they can match on
            for pattern in &matcher.patterns {
                if pattern.is_match(&copied_src) && !found_match {
                    // copy the src data again as we are iterating over multiple patterns
                    let capture_group_str = copied_src.clone();
                    if let Some(matched_token) = pattern.captures(&capture_group_str) {
                        let matched_text_token = matched_token.get(0).unwrap().as_str();

                        copied_src = copied_src[matched_text_token.len()..].to_string();

                        // push token with value, otherwise it doesnt need value
                        if matcher.name == "Keyword" {
                            tokens.push(TokenIdentifier::Keyword(matched_text_token.to_string()));
                        } else if matcher.name == "Constant" {
                            tokens.push(TokenIdentifier::Constant(matched_text_token.to_string()));
                        } else if matcher.name == "Identifier" {
                            tokens
                                .push(TokenIdentifier::Identifier(matched_text_token.to_string()));
                        } else {
                            match &matcher.token_id {
                                Some(token_id) => tokens.push(token_id.clone()),
                                _ => {}
                            }
                        }
                        found_match = true;
                        break;
                    }
                }
            }
        }

        // if we didnt find a match then our input src data is most likely flawed
        if !found_match {
            println!("no match found: \"{}\"", copied_src);
            std::process::exit(1);
        }
    }
}

fn parse(src_code_contents: &str) -> Option<String> {
    todo!()
}

fn code_gen(src_code_contents: &str) -> Option<String> {
    let parse_res = parse(src_code_contents);
    todo!()
}

fn init_grammer() -> Vec<TokenMatcher> {
    return vec![
        TokenMatcher {
            name: String::from("Identifier"),
            patterns: vec![Regex::new(r"^[a-zA-Z_]\w*\b").unwrap()],
            token_id: None,
        },
        TokenMatcher {
            name: String::from("Constant"),
            patterns: vec![Regex::new(r"^[0-9]+\b").unwrap()],
            token_id: None,
        },
        TokenMatcher {
            name: String::from("Keyword"),
            patterns: vec![
                Regex::new(r"^int\b").unwrap(),
                Regex::new(r"^void\b").unwrap(),
                Regex::new(r"^return\b").unwrap(),
            ],
            token_id: None,
        },
        TokenMatcher {
            name: String::from("OpenParenthesis"),
            patterns: vec![Regex::new(r"^\(").unwrap()],
            token_id: Some(TokenIdentifier::ParenthesisOpen),
        },
        TokenMatcher {
            name: String::from("CloseParenthesis"),
            patterns: vec![Regex::new(r"^\)").unwrap()],
            token_id: Some(TokenIdentifier::ParenthesisClose),
        },
        TokenMatcher {
            name: String::from("OpenBrace"),
            patterns: vec![Regex::new(r"^\{").unwrap()],
            token_id: Some(TokenIdentifier::BraceOpen),
        },
        TokenMatcher {
            name: String::from("CloseBrace"),
            patterns: vec![Regex::new(r"^\}").unwrap()],
            token_id: Some(TokenIdentifier::BraceClose),
        },
        TokenMatcher {
            name: String::from("Semicolon"),
            patterns: vec![Regex::new(r"^;").unwrap()],
            token_id: Some(TokenIdentifier::Semicolon),
        },
        // edgecase to deal with comments for the time being
        // TODO: remove me, but need a better solution to support comments
        // maybe a find replace of everything within the regex of a comment..?
        //
        TokenMatcher {
            name: String::from("Comment"),
            patterns: vec![Regex::new(r"\/\/.*").unwrap()],
            token_id: Some(TokenIdentifier::Comment),
        },
    ];
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("this program requires at least two arguments: ./compiler [file-path] [<args>]");
        std::process::exit(1);
    }

    let src_file = args.get(1).unwrap();
    let src_code_contents = fs::read_to_string(src_file).unwrap();
    let grammer = init_grammer();

    let tokens = lex(&src_code_contents, grammer).unwrap();
    for tok in tokens {
        dbg!(tok);
    }

    let command = parse_args(args[2..].to_vec()).unwrap();
    //     match command {
    //         Command::Lex => lex(&src_code_contents, grammer),
    //         Command::Parse => parse(&src_code_contents),
    //     Command::Codegen => code_gen(&src_code_contents),
    //     };
}
