use cli_kit::ansi::color_codes::{cyan, green, magenta};
use std::convert::From;
use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug, Clone)]
enum LexItem {
    Paren(char),
    Op(char),
    Char(char),
    Num(char),
    Whitespace(char),
}

// Allow for lexed tokens to be used as Strings
impl From<LexItem> for String {
    fn from(item: LexItem) -> Self {
        match item {
            LexItem::Char(i) => i.to_string(),
            LexItem::Whitespace(_) => String::from(" "),
            _ => String::from(""),
        }
    }
}

fn main() {
    let args = Cli::from_args();
    let colored_version = String::new();
    let file_content = read_to_string(args.file);
    let colored_version = parse(lex(&file_content.unwrap()), colored_version);
    print!("{}", colored_version)
}


fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                result.push(LexItem::Num(c));
            }
            '+' | '*' | '-' | '/' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            ' ' | '\t' => {
                result.push(LexItem::Whitespace(c));
                it.next();
            }
            _ => {
                result.push(LexItem::Char(c));
                it.next();
            }
        }
    }
    Ok(result)
}

fn parse(lexed_input: Result<Vec<LexItem>, String>, mut colored_version: String) -> String {
    for i in lexed_input.unwrap() {
        match i {
            LexItem::Num(n) => colored_version += &magenta(n, false, false).to_string(),
            LexItem::Op(n) => colored_version += &cyan(n, false, false).to_string(),
            LexItem::Paren(n) => colored_version += &green(n, false, false).to_string(),
            LexItem::Whitespace(_) | LexItem::Char(_) => colored_version += &String::from(i),
        };
    }
    return colored_version;
}
