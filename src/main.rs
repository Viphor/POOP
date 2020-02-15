use logos::Logos;
use poop::lexer::Token;

fn main() {
    let mut lexer = Token::lexer("fn lol(x: int) { // Comment\n  let y = 5 + x;\n}");

    assert_eq!(lexer.token, Token::Fn);
    assert_eq!(lexer.slice(), "fn");
    assert_eq!(lexer.range(), 0..2);

    lexer.advance();

    assert_eq!(lexer.token, Token::Ident);
    assert_eq!(lexer.slice(), "lol");
    assert_eq!(lexer.range(), 3..6);

    lexer.advance();

    assert_eq!(lexer.token, Token::LParen);
    assert_eq!(lexer.slice(), "(");

    lexer.advance();

    assert_eq!(lexer.token, Token::Ident);
    assert_eq!(lexer.slice(), "x");

    lexer.advance();

    assert_eq!(lexer.token, Token::Colon);
    assert_eq!(lexer.slice(), ":");

    lexer.advance();

    assert_eq!(lexer.token, Token::Ident);
    assert_eq!(lexer.slice(), "int");

    lexer.advance();

    assert_eq!(lexer.token, Token::RParen);
    assert_eq!(lexer.slice(), ")");

    lexer.advance();

    assert_eq!(lexer.token, Token::LBrace);
    assert_eq!(lexer.slice(), "{");

    lexer.advance();

    assert_eq!(lexer.token, Token::LineComment);
    //assert_eq!(lexer.slice(), "{");

    lexer.advance();

    assert_eq!(lexer.token, Token::Let);
    assert_eq!(lexer.slice(), "let");
}
