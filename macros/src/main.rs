use logos::{Logos, Lexer};

macro_rules! gen_lexer {
    ($enumName:ident, $(($token:ident,$target:literal)), *) => {
        #[derive(Logos, Debug, Clone, PartialEq, Eq)]
        enum $enumName {
            #[end]
            End,
            #[error]
            Error,
            #[token = " "]
            Whitespace,
            $(
                #[token = $target]
                $token,
            )*
        }
    };
}

macro_rules! gen_parse {
    ($enumName:ident, $funcName:ident, $(($token:ident, $ansi:literal)), *) => {
        fn $funcName(mut tokens: Lexer<$enumName, &str>) {
            while tokens.token != $enumName::End {
                match tokens.token {
                    $(
                        $enumName::$token => print!("\x1b[{}m{}\x1b[m", $ansi, tokens.slice()),
                    )*
                    _ => print!("{}", tokens.slice())
                }
                tokens.advance();
            }
        }
    };
}

fn main() {
    gen_lexer!(lexer, (KeyWord, "help"));
    gen_parse!(lexer, parser, (KeyWord, "31"));
    parser(lexer::lexer("help x f"));
    println!();   
}
