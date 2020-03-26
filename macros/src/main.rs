use logos::Logos;

macro_rules! gen_lexer {
    ($enumName:ident, $(($token:ident,$target:literal)), *) => {
        #[derive(Logos, Debug, Clone, PartialEq, Eq)]
        enum $enumName {
            #[end]
            End,
            #[error]
            Error,
            $(
                #[token = $target]
                $token,
            )*
        }
    };
}

macro_rules! applyArgs {
    () => {
        //
    };
}

fn main() {
    gen_lexer!(lexer, (f, "help"));
    let lexed = lexer::lexer("help x f");
    println!("{:#?}", lexed.token)
}
