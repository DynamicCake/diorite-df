Parser struct now has error lists
1. Lexer Errors
2. Parser Errors
EOF error can just get passed down as there only is one

fn next() -> Spanned<Token<'_>> where lexer error is Token::Invalid and pushed to error list
EOF can only appear once meaning that `Vec<UnexpectedEOF<'_>>` is not allowed

