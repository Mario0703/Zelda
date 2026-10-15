mod lexerImpl;
mod lexer_erros;

#[test]
fn givenAnIntegerTokenIntIsGenerated(){

    let tokens: Vec<Tokens> = lexerImpl::LexerImplementations("zeldaInt.zd");
    assert_ok(tokens, )
}