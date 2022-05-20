use logos::{Lexer, Logos};

use crate::{
    ast::{Value, Values},
    ATTRIBUTE_SET, ELEMENT_MAP,
};

#[test]
fn test_parse() {
    let mut lex = Token::lexer(r#"div{width: "{x}px"}"#);
    assert_eq!(lex.next(), Some(Token::Element("div")));
    assert_eq!(lex.slice(), "div{");

    assert_eq!(lex.next(), Some(Token::Attribute("width")));
    assert_eq!(lex.slice(), "width:");

    assert_eq!(
        lex.next(),
        Some(Token::Values(Values(vec![
            Value::Variable("x"),
            Value::Constant("px")
        ])))
    );
    assert_eq!(lex.slice(), r#""{x}px""#);

    assert_eq!(lex.next(), Some(Token::ClosingBrace));
    assert_eq!(lex.slice(), "}");

    assert_eq!(lex.next(), None);
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token<'a> {
    // #[regex(r##"r?#*".*"#*"##, process_value, priority = 15)]
    #[regex(r#""[^"\\]*""#, process_value)]
    #[regex(r##"r#+".*"#+"##, process_value)]
    Values(Values<'a>),

    #[regex(r"[#\w\d_]+ *:", process_attribute)]
    Attribute(&'static str),

    #[regex(r"[#\w\d_]+ *\{", process_element)]
    Element(&'a str),

    #[token(r"rsx!")]
    Rsx,

    #[token(r"{")]
    OpeningBrace,

    #[token(r"}")]
    ClosingBrace,

    #[token(r"?")]
    QuestionMark,

    #[token(r",")]
    Comma,

    #[token("#")]
    Pound,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn process_element<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'a str> {
    let text = lex.slice().trim_end_matches('{').trim_end_matches(' ');
    println!("process_element: {}", text);
    if ELEMENT_MAP.contains_key_str(text) {
        Some(text)
    } else {
        None
    }
}

fn process_attribute<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Option<&'static str> {
    let text = lex.slice().trim_end_matches(':').trim_end_matches(' ');
    println!("process_attribute: {}", text);
    ATTRIBUTE_SET.get(text).copied()
}

fn process_value<'a>(lex: &mut Lexer<'a, Token<'a>>) -> Values<'a> {
    let mut text = lex.slice();
    if let Some((_, a)) = text.split_once('"') {
        text = a;
    }
    if let Some((b, _)) = text.rsplit_once('"') {
        text = b;
    }
    Values(Value::lexer(text).collect())
}
