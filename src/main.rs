use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
  #[token("#")]
  CompilerDirective,
  #[token("  ")]
  Indent,

  #[token("import")]
  Import,
  #[token("to")]
  Fn,

  #[token("&=")]
  ChainAssign,
  #[token("&")]
  Chain,
  #[token("=")]
  Assign,

  #[token(".")]
  PropertySet,

  #[token("::")]
  DomainSep,
  #[token(",")]
  ListSep,

  #[token("\"")]
  StringStartStop,

  #[token("{")]
  CompileAllStart,
  #[token("}")]
  CompileAllStop,
  #[token("(")]
  GroupStart,
  #[token(")")]
  GroupStop,

  #[regex("([a-z0-9-]+)")]
  ValueIdent,
  #[regex("([A-Z])([a-z0-0-]+)")]
  ClassIdent,

  #[error]
  #[regex(r"[ \f\n]+", logos::skip)]
  Error
}

fn main() {
  let text = std::fs::read_to_string("src/test.tang").unwrap();
  let mut lex = Token::lexer(&text);

  while let Some(tk) = lex.next() {
    if tk == Token::Error {
        println!("{:?}: {:?}", tk, lex.slice())
    }
  }
}