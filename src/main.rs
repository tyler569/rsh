use logos::Logos;

#[derive(Debug, Logos)]
enum Token<'a> {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),

    #[regex(r#""(?:\\"|[^"])*""#)]
    InterpolatedString(&'a str),

    #[regex(r#"'[^']*'"#)]
    RawString(&'a str),

    #[regex(r"\$\w+")]
    DollarVariable(&'a str),

    #[token("$")]
    Dollar,

    #[token("|")]
    Pipe,
    #[token("<")]
    RedirectIn,
    #[token(">")]
    RedirectOut,

    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("=")]
    Equal,

    #[error]
    #[regex(r"[ \t\n\f]+" ,logos::skip)]
    Error,
}

fn print(s: &str) {
    let lex = Token::lexer(s);
    for token in lex {
        println!("{:?}", token);
    }
    println!();
}

fn main() {
    println!("Hello, world!");

    print(r#"echo $HELLO | "ls -al" <(foo)"#);
    print(r#"echo hello world"#);
    print(r#"echo "hello world""#);
    print(r#"echo 'hello world'"#);
    print(r#"echo $HELLO_WORLD"#);
    print(r#"echo $(hello world)"#);
    print(r#"echo hello | world"#);
    print(r#"echo $HELLO | world"#);
    print(r#"echo hello > world"#);
    print(r#"echo <hello >world"#);
    print(r#"echo "hello" "world""#);
    print(r#"echo "$HELLO" "$WORLD""#);
    print(r#"HELLO=WORLD"#);
}
